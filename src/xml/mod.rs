//! Module for deserializing and extracting information from XML messages.

// Schemas - Generated via [xml_schema_generator](https://thomblin.github.io/xml_schema_generator/)
pub mod av_transport;
pub mod rendering_control;

pub use av_transport::AVTransport;
pub use rendering_control::RenderingControl;
