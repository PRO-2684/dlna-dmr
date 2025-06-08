//! # `dlna-dmr` library crate
//!
//! If you are reading this, you are reading the documentation for the `dlna-dmr` library crate. For the cli, kindly refer to the README file.
//!
//! ## Overview
//!
//! This crate provides a framework for building a Digital Media Renderer (DMR). It only provides the functionality of accepting commands from a Digital Media Controller (DMC), and how to handle them will be left to you to implement.
//!
//! ## Usage
//!
//! To build your DMR, you'll first need to implement the [`HTTPServer`] trait, which describes how to handle various commands:
//!
//! ```rust
//! use dlna_dmr::HTTPServer;
//!
//! struct MyDMR {}
//!
//! impl HTTPServer for MyDMR {
//!   // Refer to the documentation of `HTTPServer` on how to implement.
//! }
//! ```
//!
//! Then, you simply implement the [`DMR`] trait:
//!
//! ```rust
//! use dlna_dmr::{DMR, HTTPServer};
//! #
//! # struct MyDMR {}
//! #
//! # impl HTTPServer for MyDMR {
//! # }
//! impl DMR for MyDMR {}
//! ```
//!
//! To start your DMR, call the method [`DMR::run`] with an option:
//!
//! ```rust
//! use dlna_dmr::{DMR, DMROptions, HTTPServer};
//! use std::sync::Arc;
//! #
//! # struct MyDMR {}
//! #
//! # impl HTTPServer for MyDMR {
//! # }
//! # impl DMR for MyDMR {}
//!
//! # async fn run() { // This function won't be run intentionally
//!     // Instantiate `MyDMR`
//!     let dmr = MyDMR {};
//!     let dmr = Box::leak(Box::new(dmr));
//!     // Use default config (Refer to documentation of `DMROptions` on configuration)
//!     let options = DMROptions::default();
//!     // Running the DMR until Ctrl-C is pressed.
//!     dmr.run(Arc::new(options)).await.unwrap();
//! # }
//! ```

#![deny(missing_docs)]
#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, reason = "Dependencies' requirements")]

mod defaults;
mod http;
mod ssdp;
pub mod xml;

pub use axum::response::Response;
pub use http::HTTPServer;
use log::{error, info};
use serde::{Deserialize, Serialize};
use ssdp::SSDPServer;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
    io::Result as IoResult,
};

/// Options for a DMR instance.
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

impl Default for DMROptions {
    fn default() -> Self {
        Self {
            ip: defaults::ip(),
            ssdp_port: defaults::ssdp_port(),
            http_port: defaults::http_port(),
            uuid: defaults::uuid(),
            friendly_name: defaults::friendly_name(),
            model_name: defaults::model_name(),
            model_description: defaults::model_description(),
            model_url: defaults::model_url(),
            manufacturer: defaults::manufacturer(),
            manufacturer_url: defaults::manufacturer_url(),
            serial_number: defaults::serial_number(),
        }
    }
}

/// A trait for DMR instances.
pub trait DMR: HTTPServer {
    /// Create and run the DMR instance, stopping when Ctrl-C is pressed.
    fn run(&'static self, options: Arc<DMROptions>) -> impl Future<Output = IoResult<()>> + Send
    where
        Self: Sync,
    {async {
        let address = SocketAddrV4::new(options.ip, options.ssdp_port);
        let ssdp = SSDPServer::new(
            address,
            options.uuid.clone(),
            options.http_port,
        )
        .await?;

        tokio::select! {
            _ = ssdp.keep_alive() => {}
            _ = ssdp.run() => {}
            r = self.run_http(options) => {
                if let Err(e) = r {
                    error!("IO Error while running HTTP server: {e}");
                }
            }
            r = tokio::signal::ctrl_c() => {
                if let Err(e) = r {
                    error!("IO Error while waiting for Ctrl-C: {e}");
                }
            }
        }

        ssdp.stop().await;

        info!("DMR stopped");
        Ok(())
    } }
}
