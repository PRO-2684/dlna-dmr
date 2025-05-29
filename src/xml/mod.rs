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
            // let deserialized: AVTransportEnvelope = deserialize(text);
            match deserialize::<AVTransportEnvelope>(text) {
                Ok(deserialized) => {
                    match deserialized.value() {
                        AVTransport::USetAvTransportUri(set) => {
                            result.push(format!("AVTransport::USetAvTransportUri current_uri: {}", set.current_uri));
                        },
                        AVTransport::UPlay(play) => {
                            result.push(format!("AVTransport::UPlay speed: {}", play.speed));
                        },
                    }
                },
                Err(e) => {
                    warn!("Failed to deserialize XML: {e}");
                },
            }
        },
        _ => {},
    };

    result
}


