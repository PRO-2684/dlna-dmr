#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use axum::{http::StatusCode, response::IntoResponse};
use dlna_dmr::{
    DMR, DMROptions, HTTPServer,
    xml::{AVTransport, RenderingControl},
};
use log::{info, warn};
use quick_xml::DeError;
use std::{
    io::{Error, ErrorKind, Result as IoResult},
    sync::Arc,
};

struct DummyDMR {}

impl HTTPServer for DummyDMR {
    async fn post_av_transport(
        &self,
        av_transport: Result<AVTransport, DeError>,
    ) -> impl IntoResponse {
        match av_transport {
            Ok(av_transport) => match av_transport {
                AVTransport::SetAVTransportURI(set) => info!(
                    "AVTransport::SetAvTransportUri current_uri: {}",
                    set.current_uri
                ),
                AVTransport::SetNextAVTransportURI(set) => info!(
                    "AVTransport::SetNextAvTransportUri next_uri: {}",
                    set.next_uri
                ),
                AVTransport::Stop(_) => info!("AVTransport::Stop"),
                AVTransport::Play(play) => info!("AVTransport::Play speed: {}", play.speed),
                AVTransport::Pause(_) => info!("AVTransport::Pause"),
                AVTransport::Next(_) => info!("AVTransport::Next"),
                AVTransport::Previous(_) => info!("AVTransport::Previous"),
                _ => {}
            },
            Err(e) => warn!("Failed to deserialize `/AVTransport` XML: {e}"),
        };
        StatusCode::METHOD_NOT_ALLOWED
    }

    async fn post_rendering_control(
        &self,
        rendering_control: Result<RenderingControl, DeError>,
    ) -> impl IntoResponse {
        match rendering_control {
            Ok(rendering_control) => match rendering_control {
                RenderingControl::SelectPreset(select) => info!(
                    "RenderingControl::SelectPreset preset: {}",
                    select.preset_name
                ),
                RenderingControl::SetMute(set) => info!(
                    "RenderingControl::SetMute channel: {}, desired_mute: {}",
                    set.channel, set.desired_mute
                ),
                RenderingControl::SetVolume(set) => info!(
                    "RenderingControl::SetVolume channel: {}, desired_volume: {}",
                    set.channel, set.desired_volume
                ),
                _ => {}
            },
            Err(e) => {
                warn!("Failed to deserialize `/RenderingControl` XML: {e}");
            }
        }
        StatusCode::METHOD_NOT_ALLOWED
    }
}

impl DMR for DummyDMR {}

#[tokio::main]
async fn main() -> IoResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Load and parse configuration
    let config = if let Some(arg) = std::env::args().nth(1) {
        info!("Using configuration file: {arg}");
        std::fs::read_to_string(arg)?
    } else {
        info!("No configuration file provided, using default settings");
        "".to_string()
    };
    let options: DMROptions = toml::from_str(&config).map_err(|e| {
        eprintln!("Failed to parse configuration: {e}");
        Error::new(ErrorKind::InvalidData, e)
    })?;

    let dmr = DummyDMR {};
    let dmr = Box::leak(Box::new(dmr));

    // Start the DMR, stopping when Ctrl-C is pressed.
    dmr.run(Arc::new(options)).await
}
