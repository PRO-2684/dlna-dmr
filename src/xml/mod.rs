//! Module for deserializing and extracting information from XML messages.

// Schemas - Generated via [xml_schema_generator](https://thomblin.github.io/xml_schema_generator/)
pub mod av_transport;
pub mod rendering_control;

use super::Endpoint;
use av_transport::AVTransport;
use rendering_control::RenderingControl;
use log::warn;
use std::str::FromStr;

/// Extracts potentially useful information from given text.
#[must_use]
pub fn extract(path: Endpoint, text: &str) -> Option<String> {
    match path {
        Endpoint::AVTransport => match AVTransport::from_str(text) {
            Ok(av_transport) => match av_transport {
                AVTransport::SetAVTransportURI(set) => Some(format!(
                    "AVTransport::SetAvTransportUri current_uri: {}",
                    set.current_uri
                )),
                AVTransport::SetNextAVTransportURI(set) => Some(format!(
                    "AVTransport::SetNextAvTransportUri next_uri: {}",
                    set.next_uri
                )),
                AVTransport::Stop(_) => Some("AVTransport::Stop".to_string()),
                AVTransport::Play(play) => Some(format!("AVTransport::Play speed: {}", play.speed)),
                AVTransport::Pause(_) => Some("AVTransport::Pause".to_string()),
                AVTransport::Next(_) => Some("AVTransport::Next".to_string()),
                AVTransport::Previous(_) => Some("AVTransport::Previous".to_string()),
                _ => None,
            },
            Err(e) => {
                warn!("Failed to deserialize `/AVTransport` XML: {e}");
                None
            }
        },
        Endpoint::RenderingControl => match RenderingControl::from_str(text) {
            Ok(rendering_control) => match rendering_control {
                RenderingControl::SelectPreset(select) => Some(format!(
                    "RenderingControl::SelectPreset preset: {}",
                    select.preset_name
                )),
                RenderingControl::SetMute(set) => Some(format!(
                    "RenderingControl::SetMute channel: {}, desired_mute: {}",
                    set.channel, set.desired_mute
                )),
                RenderingControl::SetVolume(set) => Some(format!(
                    "RenderingControl::SetVolume channel: {}, desired_volume: {}",
                    set.channel, set.desired_volume
                )),
                _ => None,
            },
            Err(e) => {
                warn!("Failed to deserialize `/RenderingControl` XML: {e}");
                return None;
            }
        },
        _ => None,
    }
}
