//! Module for deserializing and extracting information from XML messages.

// Schemas - Generated via [xml_schema_generator](https://thomblin.github.io/xml_schema_generator/)
pub mod av_transport;
pub mod rendering_control;

use super::Endpoint;
use av_transport::AVTransport;
use log::warn;
use rendering_control::RenderingControl;
use std::str::FromStr;

/// Summarizes the action, if any.
trait ActionSummary {
    /// Returns an optional string summarizing the action.
    fn summary(&self) -> Option<String> {
        None
    }
}

/// Extracts potentially useful information from given text.
#[must_use]
pub fn extract(path: Endpoint, text: &str) -> Option<String> {
    match path {
        Endpoint::AVTransport => match AVTransport::from_str(text) {
            Ok(av_transport) => av_transport.summary(),
            Err(e) => {
                warn!("Failed to deserialize `/AVTransport` XML: {e}");
                None
            }
        },
        Endpoint::RenderingControl => match RenderingControl::from_str(text) {
            Ok(rendering_control) => rendering_control.summary(),
            Err(e) => {
                warn!("Failed to deserialize `/RenderingControl` XML: {e}");
                return None;
            }
        },
        _ => None,
    }
}
