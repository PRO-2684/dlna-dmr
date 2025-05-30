//! Module for deserializing and extracting information from XML messages.

// Schemas - Generated via [xml_schema_generator](https://thomblin.github.io/xml_schema_generator/)
pub mod av_transport;
pub mod rendering_control;
use av_transport::{AVTransport, AVTransportEnvelope};
use log::warn;
use quick_xml::de::from_str as deserialize;

use super::Endpoint;

/// Extracts potentially useful information from given text.
#[must_use]
pub fn extract(path: Endpoint, text: &str) -> Option<String> {
    match path {
        Endpoint::AVTransport => match deserialize::<AVTransportEnvelope>(text) {
            Ok(deserialized) => match deserialized.into_inner() {
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
        _ => None,
    }
}
