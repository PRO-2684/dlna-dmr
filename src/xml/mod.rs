//! Module for deserializing and extracting information from XML messages.

// Schemas - Generated via [xml_schema_generator](https://thomblin.github.io/xml_schema_generator/)
pub mod av_transport;
use av_transport::{AVTransportEnvelope, AVTransport};
use log::warn;
use quick_xml::{de::from_str as deserialize};

/// Extracts potentially useful information from given text.
pub fn extract(path: &str, text: &str) -> Vec<String> {
    let mut result = Vec::new();
    match path {
        "/AVTransport" => {
            match deserialize::<AVTransportEnvelope>(text) {
                Ok(deserialized) => {
                    match deserialized.into_inner() {
                        AVTransport::SetAVTransportURI(set) => {
                            result.push(format!("AVTransport::SetAvTransportUri current_uri: {}", set.current_uri));
                        },
                        AVTransport::SetNextAVTransportURI(set) => {
                            result.push(format!("AVTransport::SetNextAvTransportUri next_uri: {}", set.next_uri));
                        },
                        AVTransport::Stop(_) => {
                            result.push("AVTransport::Stop".to_string());
                        },
                        AVTransport::Play(play) => {
                            result.push(format!("AVTransport::Play speed: {}", play.speed));
                        },
                        AVTransport::Pause(_) => {
                            result.push("AVTransport::Pause".to_string());
                        },
                        AVTransport::Next(_) => {
                            result.push("AVTransport::Next".to_string());
                        },
                        AVTransport::Previous(_) => {
                            result.push("AVTransport::Previous".to_string());
                        },
                        _ => {}
                    }
                },
                Err(e) => {
                    warn!("Failed to deserialize `/AVTransport` XML: {e}");
                },
            }
        },
        _ => {},
    };

    result
}


