//! # `dlna-dmr` library crate
//!
//! If you are reading this, you are reading the documentation for the `dlna-dmr` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Dependencies' requirements")]

mod http;
mod ssdp;
pub mod xml;

pub use http::{HTTPServer, Response};
use local_ip_address::local_ip;
use log::{error, info};
use ssdp::SSDPServer;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddrV4},
    sync::{Arc, atomic::AtomicBool},
};

/// Options for creating a new [`DMR`] instance.
#[derive(Debug, Clone)]
pub struct DMROptions {
    /// Local IP.
    pub ip: Ipv4Addr,
    /// The SSDP server port.
    pub ssdp_port: u16,
    /// The HTTP server port.
    pub http_port: u16,
    /// The UUID of the DMR instance.
    pub uuid: String,
    /// Friendly name of the DMR instance.
    pub friendly_name: String,
    /// Model name of the DMR instance.
    pub model_name: String,
    /// Model description of the DMR instance.
    pub model_description: String,
    /// Model URL of the DMR instance.
    pub model_url: String,
    /// Manufacturer of the DMR instance.
    pub manufacturer: String,
    /// Manufacturer URL of the DMR instance.
    pub manufacturer_url: String,
    /// Serial number of the DMR instance.
    pub serial_number: String,
}

impl Default for DMROptions {
    /// Creates a default set of options for the DMR instance.
    fn default() -> Self {
        let ip = local_ip().expect("Failed to get local IP address");
        let IpAddr::V4(ip) = ip else {
            panic!("IPv6 is not supported");
        };
        let uuid = uuid::Uuid::new_v4().to_string();
        Self {
            ip,
            ssdp_port: 1900,
            http_port: 8080, // Default HTTP port
            uuid,
            friendly_name: "Dummy Renderer".to_string(),
            model_name: "Dummy Model".to_string(),
            model_description: "A dummy DLNA DMR".to_string(),
            model_url: "http://example.com/dummy_model".to_string(),
            manufacturer: "Dummy Manufacturer".to_string(),
            manufacturer_url: "http://example.com/manufacturer".to_string(),
            serial_number: "12345678-1234-5678-1234-567812345678".to_string(),
        }
    }
}

/// A trait for DMR instances.
pub trait DMR: HTTPServer {
    /// Create and run the DMR instance, blocking current thread.
    ///
    /// ## Stopping
    ///
    /// To stop the DMR, set the `running` signal, as you've passed in the [`new`](Self::new) method, to `false`:
    ///
    /// ```rust ignore
    /// use std::sync::atomic::Ordering;
    /// running.store(false, Ordering::SeqCst);
    /// ```
    fn run(&self, options: DMROptions, running: Arc<AtomicBool>)
    where
        Self: Sync,
    {
        let address = SocketAddrV4::new(options.ip, options.ssdp_port);
        let ssdp = SSDPServer::new(
            address,
            options.uuid.clone(),
            options.http_port,
            running.clone(),
        )
        .expect("Failed to create SSDP server");
        if let Err(e) = ssdp.alive() {
            error!("Error broadcasting alive message: {e}");
        };

        // Scoped thread
        std::thread::scope(|s| {
            // Start the SSDP server
            s.spawn(|| ssdp.run());
            // Start the HTTP server
            s.spawn(|| self.run_http(options, running));
            info!("DMR started");
        });

        info!("DMR stopped");
    }
}
