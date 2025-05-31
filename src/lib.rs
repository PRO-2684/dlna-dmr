//! # `dlna-dmr` library crate
//!
//! If you are reading this, you are reading the documentation for the `dlna-dmr` library crate. For the cli, kindly refer to the README file.

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Dependencies' requirements")]

mod defaults;
mod http;
mod ssdp;
pub mod xml;

pub use http::{HTTPServer, Response};
use log::{error, info};
use serde::{Deserialize, Serialize};
use ssdp::SSDPServer;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::{Arc, atomic::AtomicBool},
};

/// Options for creating a new [`DMR`] instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMROptions {
    /// Local IP.
    #[serde(default = "defaults::ip")]
    pub ip: Ipv4Addr,
    /// The SSDP server port.
    #[serde(default = "defaults::ssdp_port")]
    pub ssdp_port: u16,
    /// The HTTP server port.
    #[serde(default = "defaults::http_port")]
    pub http_port: u16,
    /// The UUID of the DMR instance.
    #[serde(default = "defaults::uuid")]
    pub uuid: String,
    /// Friendly name of the DMR instance.
    #[serde(default = "defaults::friendly_name")]
    pub friendly_name: String,
    /// Model name of the DMR instance.
    #[serde(default = "defaults::model_name")]
    pub model_name: String,
    /// Model description of the DMR instance.
    #[serde(default = "defaults::model_description")]
    pub model_description: String,
    /// Model URL of the DMR instance.
    #[serde(default = "defaults::model_url")]
    pub model_url: String,
    /// Manufacturer of the DMR instance.
    #[serde(default = "defaults::manufacturer")]
    pub manufacturer: String,
    /// Manufacturer URL of the DMR instance.
    #[serde(default = "defaults::manufacturer_url")]
    pub manufacturer_url: String,
    /// Serial number of the DMR instance.
    #[serde(default = "defaults::serial_number")]
    pub serial_number: String,
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
