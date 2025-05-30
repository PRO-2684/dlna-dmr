//! # `dlna-dmr` library crate
//!
//! If you are reading this, you are reading the documentation for the `dlna-dmr` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Dependencies' requirements")]

mod http;
mod ssdp;
pub mod xml;

use http::HTTPServer;
use local_ip_address::local_ip;
use log::info;
use ssdp::SSDPServer;
use std::{
    io::Result,
    net::{IpAddr, SocketAddrV4},
    sync::{Arc, atomic::AtomicBool},
};
use xml::extract;

/// Options for creating a new [`DMR`] instance.
#[derive(Debug, Clone)]
pub struct DMROptions {
    /// The SSDP server address.
    pub address: SocketAddrV4,
    /// The UUID of the DMR instance.
    pub uuid: String,
    /// The HTTP port for the DMR instance.
    pub http_port: u16,
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
        let address = local_ip().expect("Failed to get local IP address");
        let address = match address {
            IpAddr::V4(address) => SocketAddrV4::new(address, 1900),
            IpAddr::V6(_) => {
                panic!("IPv6 is not supported for SSDP in this implementation");
            }
        };
        let uuid = uuid::Uuid::new_v4().to_string();
        Self {
            address,
            uuid,
            http_port: 8080, // Default HTTP port
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

/// A dummy DLNA DMR (Digital Media Renderer) instance.
pub struct DMR {
    /// The SSDP server instance.
    ssdp: SSDPServer,
    /// The HTTP server instance.
    http: HTTPServer,
}

impl DMR {
    /// Creates a new DMR instance with given options and running signal.
    ///
    /// ## Panics
    ///
    /// Panics if the SSDP server cannot be created, like socket binding failure.
    pub fn new(options: DMROptions, running: Arc<AtomicBool>) -> Self {
        let ssdp = SSDPServer::new(
            options.address,
            options.uuid.clone(),
            options.http_port,
            running.clone(),
        )
        .expect("Failed to create SSDP server");
        let http = HTTPServer::new(options, running);
        Self { ssdp, http }
    }

    /// Starts the DMR instance, blocking current thread.
    ///
    /// ## Stopping
    ///
    /// To stop the DMR, set the `running` signal, as you've passed in the [`new`](Self::new) method, to `false`:
    ///
    /// ```rust ignore
    /// use std::sync::atomic::Ordering;
    /// running.store(false, Ordering::SeqCst);
    /// ```
    ///
    /// ## Errors
    ///
    /// Returns an error if alive notification fails.
    pub fn start(&self) -> Result<()> {
        self.ssdp.alive()?;

        // Scoped thread
        std::thread::scope(|s| {
            // Start the SSDP server
            s.spawn(|| self.ssdp.run());
            // Start the HTTP server
            s.spawn(|| self.http.run());
            info!("DMR started");
        });

        info!("DMR stopped");
        Ok(())
    }
}
