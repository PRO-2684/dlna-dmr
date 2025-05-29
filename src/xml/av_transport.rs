//! Module for deserializing and extracting information from AVTransport XML messages. See [`AVTransportEnvelope`] and [`AVTransport`] for more details.

#![allow(missing_docs, reason = "Fields are self-explanatory")]

use serde::{Serialize, Deserialize};

/// The envelope structure for AVTransport XML messages.
///
/// Usually, once deserialized, you'll call [`AVTransportEnvelope::value`] to consume it and get the actual content of the message, which you could match against the [`AVTransport`] enum to determine the specific type of AVTransport action.
///
/// ## Example
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use quick_xml::de::from_str;
/// use dlna_dmr::xml::av_transport::{AVTransportEnvelope, AVTransport, USetAvtransportUri, UPlay};
///
/// let xml = r#"<?xml version="1.0"?>
/// <s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/" s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">
///     <s:Body>
///         <u:Play xmlns:u="urn:schemas-upnp-org:service:AVTransport:1">
///             <Speed>1</Speed>
///             <InstanceID>0</InstanceID>
///         </u:Play>
///     </s:Body>
/// </s:Envelope>
/// "#;
/// let deserialized: AVTransportEnvelope = from_str(xml).expect("Failed to deserialize XML");
/// let play_action = match deserialized.value() {
///     AVTransport::UPlay(play) => play,
///     _ => panic!("Expected UPlay variant"),
/// };
/// assert_eq!(play_action.instance_id, "0");
/// assert_eq!(play_action.speed, "1");
#[derive(Serialize, Deserialize, Debug)]
pub struct AVTransportEnvelope {
    #[serde(rename = "@encodingStyle")]
    pub s_encoding_style: String,
    #[serde(rename = "@xmlns:s")]
    pub xmlns_s: String,
    #[serde(rename = "Body")]
    pub s_body: SBody,
}

impl AVTransportEnvelope {
    /// Take ownership of the [`AVTransport`] value contained in the envelope, consuming the envelope.
    pub fn value(self) -> AVTransport {
        self.s_body.content
    }
}

/// Container structure.
#[derive(Serialize, Deserialize, Debug)]
pub struct SBody {
    #[serde(rename = "$value")]
    content: AVTransport,
}

/// Different types of action that can be invoked in the AVTransport service.
#[derive(Serialize, Deserialize, Debug)]
pub enum AVTransport {
    #[serde(rename = "SetAVTransportURI")]
    USetAvTransportUri(USetAvtransportUri),
    #[serde(rename = "Play")]
    UPlay(UPlay),
    // TODO: Complete the list
}

/// Arguments for the `SetAVTransportURI` action in [`AVTransport`].
#[derive(Serialize, Deserialize, Debug)]
pub struct USetAvtransportUri {
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    #[serde(rename = "InstanceID")]
    pub instance_id: String,
    #[serde(rename = "CurrentURI")]
    pub current_uri: String,
    #[serde(rename = "CurrentURIMetaData")]
    pub current_uri_meta_data: String,
}

/// Arguments for the `Play` action in [`AVTransport`].
#[derive(Serialize, Deserialize, Debug)]
pub struct UPlay {
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    #[serde(rename = "Speed")]
    pub speed: String,
    #[serde(rename = "InstanceID")]
    pub instance_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    use quick_xml::de::from_str as deserialize;

    #[test]
    fn test_set_av_transport_uri() {
        let xml = read_to_string("tests/AVTransport/SetAVTransportURI.xml")
            .expect("Failed to read XML file");
        let deserialized: AVTransportEnvelope = deserialize(&xml).expect("Failed to deserialize XML");
        eprintln!("{deserialized:#?}");

        let set_action = match deserialized.value() {
            AVTransport::USetAvTransportUri(set) => set,
            _ => panic!("Expected USetAvTransportUri variant"),
        };
        assert_eq!(set_action.instance_id, "0");
        assert_eq!(set_action.current_uri, "http://example.com/sample.mp4?param1=a&param2=b");
        assert_eq!(set_action.current_uri_meta_data, "");
    }

    #[test]
    fn test_play() {
        let xml = read_to_string("tests/AVTransport/Play.xml")
            .expect("Failed to read XML file");
        let deserialized: AVTransportEnvelope = deserialize(&xml).expect("Failed to deserialize XML");
        eprintln!("{deserialized:#?}");

        let play_action = match deserialized.value() {
            AVTransport::UPlay(play) => play,
            _ => panic!("Expected UPlay variant"),
        };
        assert_eq!(play_action.instance_id, "0");
        assert_eq!(play_action.speed, "1");
    }
}
