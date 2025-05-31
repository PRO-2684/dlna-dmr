#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::cargo)]

use dlna_dmr::{
    DMR, DMROptions, HTTPServer, Response,
    xml::{AVTransport, RenderingControl},
};
use log::{info, warn};
use quick_xml::DeError;
use std::{
    io::Result as IoResult,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tiny_http::StatusCode;

struct DummyDMR {}

impl HTTPServer for DummyDMR {
    fn post_av_transport(&self, av_transport: Result<AVTransport, DeError>) -> Response {
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
        Response::from_string("").with_status_code(StatusCode(405))
    }

    fn post_rendering_control(
        &self,
        rendering_control: Result<RenderingControl, DeError>,
    ) -> Response {
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
        Response::from_string("").with_status_code(StatusCode(405))
    }
}

impl DMR for DummyDMR {}

fn main() -> IoResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let options = DMROptions::default();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let dmr = DummyDMR {};

    // Set up Ctrl-C handler before starting the servers
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Start the DMR, which will block until stopped
    dmr.run(options, running);
    Ok(())
}
