//! Module for deserializing and extracting information from `AVTransport` XML messages. See [`AVTransportEnvelope`] and [`AVTransport`] for more details.

#![allow(missing_docs, reason = "Fields are self-explanatory")]

use std::fmt::Display;

use quick_xml::{DeError, de};
use serde::{Deserialize, Serialize};

/// The envelope structure for `AVTransport` XML messages.
///
/// Usually, once deserialized, you'll call [`AVTransportEnvelope::into_inner`] to consume it and get the actual content of the message, which you could match against the [`AVTransport`] enum to determine the specific type of `AVTransport` action. For an even simpler usage, [`AVTransport`] implements `FromStr`, allowing you to directly deserialize from a XML envelope string.
///
/// ## Example
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use quick_xml::de::from_str;
/// use dlna_dmr::xml::av_transport::{AVTransportEnvelope, AVTransport, PlaySpeed};
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
/// let play_action = match deserialized.into_inner() {
///     AVTransport::Play(play) => play,
///     _ => panic!("Expected Play variant"),
/// };
/// assert_eq!(play_action.instance_id, 0);
/// assert_eq!(play_action.speed, PlaySpeed::One);
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    #[must_use]
    pub fn into_inner(self) -> AVTransport {
        self.s_body.content
    }
}

/// Container structure.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SBody {
    #[serde(rename = "$value")]
    content: AVTransport,
}

/// Different types of action that can be invoked in the `AVTransport` service. Can be directly parsed from an XML envelope string, IGNORING the outer envelope structure.
///
/// ## Example
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use dlna_dmr::xml::av_transport::{AVTransport, PlaySpeed};
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
/// let av_transport: AVTransport = xml.parse().expect("Failed to parse AVTransport");
/// let play_action = match av_transport {
///     AVTransport::Play(play) => play,
///     _ => panic!("Expected Play variant"),
/// };
/// assert_eq!(play_action.instance_id, 0);
/// assert_eq!(play_action.speed, PlaySpeed::One);
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AVTransport {
    SetAVTransportURI(SetAVTransportURI),
    SetNextAVTransportURI(SetNextAVTransportURI),
    GetMediaInfo(Simple),
    GetTransportInfo(Simple),
    GetPositionInfo(Simple),
    GetDeviceCapabilities(Simple),
    GetTransportSettings(Simple),
    Stop(Simple),
    Play(Play),
    Pause(Simple),
    Seek(Seek),
    Next(Simple),
    Previous(Simple),
    GetCurrentTransportActions(Simple),
}

impl std::str::FromStr for AVTransport {
    type Err = DeError;
    /// Deserialize from an envelope, IGNORING the outer envelope structure.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let envelope: AVTransportEnvelope = de::from_str(s)?;
        Ok(envelope.into_inner())
    }
}

/// Arguments for the `SetAVTransportURI` action in [`AVTransport`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetAVTransportURI {
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    #[serde(rename = "InstanceID")]
    pub instance_id: u32,
    #[serde(rename = "CurrentURI")]
    pub current_uri: String,
    #[serde(rename = "CurrentURIMetaData")]
    pub current_uri_meta_data: String,
}

/// Arguments for the `SetNextAVTransportURI` action in [`AVTransport`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetNextAVTransportURI {
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    #[serde(rename = "InstanceID")]
    pub instance_id: u32,
    #[serde(rename = "NextURI")]
    pub next_uri: String,
    #[serde(rename = "NextURIMetaData")]
    pub next_uri_meta_data: String,
}

/// A single `instance_id` argument. For the following actions in [`AVTransport`]:
///
/// - `GetMediaInfo`
/// - `GetTransportInfo`
/// - `GetPositionInfo`
/// - `GetDeviceCapabilities`
/// - `GetTransportSettings`
/// - `Stop`
/// - `Pause`
/// - `Next`
/// - `Previous`
/// - `GetCurrentTransportActions`
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Simple {
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    #[serde(rename = "InstanceID")]
    pub instance_id: u32,
}

/// Arguments for the `Play` action in [`AVTransport`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Play {
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    #[serde(rename = "Speed")]
    pub speed: PlaySpeed,
    #[serde(rename = "InstanceID")]
    pub instance_id: u32,
}

/// Possible values for the [`speed`](`Play::speed`) field of the [`Play`] action in [`AVTransport`].
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaySpeed {
    #[serde(rename = "1")]
    One,
}

impl Display for PlaySpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::One => write!(f, "1"),
        }
    }
}

/// Arguments for the `Seek` action in [`AVTransport`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Seek {
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    #[serde(rename = "Target")]
    pub target: String,
    #[serde(rename = "Unit")]
    pub unit: SeekUnit,
    #[serde(rename = "InstanceID")]
    pub instance_id: u32,
}

/// Possible values for the [`unit`](`Seek::unit`) field of the [`Seek`] action in [`AVTransport`].
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeekUnit {
    #[serde(rename = "ABS_COUNT")]
    AbsCount,
    #[serde(rename = "TRACK_NR")]
    TrackNr,
    #[serde(rename = "REL_TIME")]
    RelTime,
}

impl Display for SeekUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AbsCount => write!(f, "ABS_COUNT"),
            Self::RelTime => write!(f, "TRACK_NR"),
            Self::TrackNr => write!(f, "REL_TIME"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn get_xml(path: &str) -> AVTransport {
        let xml =
            read_to_string(format!("tests/AVTransport/{path}")).expect("Failed to read XML file");
        let av_transport: AVTransport = xml.parse().expect("Failed to parse AVTransport");
        av_transport
    }

    #[test]
    fn test_set_av_transport_uri() {
        let av_transport: AVTransport = get_xml("SetAVTransportURI.xml");
        // let set_action = match av_transport {
        //     AVTransport::SetAVTransportURI(set) => set,
        //     _ => panic!("Expected SetAVTransportURI variant"),
        // };
        let AVTransport::SetAVTransportURI(set_action) = av_transport else {
            panic!("Expected SetAVTransportURI variant")
        };
        assert_eq!(set_action.instance_id, 0);
        assert_eq!(
            set_action.current_uri,
            "http://example.com/sample.mp4?param1=a&param2=b"
        );
        assert_eq!(set_action.current_uri_meta_data, "");
    }

    #[test]
    fn test_set_next_av_transport_uri() {
        let av_transport: AVTransport = get_xml("SetNextAVTransportURI.xml");
        let AVTransport::SetNextAVTransportURI(set_action) = av_transport else {
            panic!("Expected SetNextAVTransportURI variant")
        };
        assert_eq!(set_action.instance_id, 0);
        assert_eq!(
            set_action.next_uri,
            "http://example.com/sample.mp4?param1=a&param2=b"
        );
        assert_eq!(set_action.next_uri_meta_data, "");
    }

    #[test]
    fn test_get_media_info() {
        let av_transport: AVTransport = get_xml("GetMediaInfo.xml");
        let AVTransport::GetMediaInfo(get_action) = av_transport else {
            panic!("Expected GetMediaInfo variant")
        };
        assert_eq!(get_action.instance_id, 0);
    }

    #[test]
    fn test_get_transport_info() {
        let av_transport: AVTransport = get_xml("GetTransportInfo.xml");
        let AVTransport::GetTransportInfo(get_action) = av_transport else {
            panic!("Expected GetTransportInfo variant")
        };
        assert_eq!(get_action.instance_id, 0);
    }

    // Other tests for GetPositionInfo, GetDeviceCapabilities, GetTransportSettings, Stop, Pause, Next, Previous, and GetCurrentTransportActions would follow a similar pattern, thus skipping them for brevity.

    #[test]
    fn test_play() {
        let av_transport: AVTransport = get_xml("Play.xml");
        let AVTransport::Play(play_action) = av_transport else {
            panic!("Expected Play variant")
        };
        assert_eq!(play_action.instance_id, 0);
        assert_eq!(play_action.speed, PlaySpeed::One);
    }

    #[test]
    fn test_seek() {
        let av_transport: AVTransport = get_xml("Seek.xml");
        let AVTransport::Seek(seek_action) = av_transport else {
            panic!("Expected Seek variant")
        };
        assert_eq!(seek_action.instance_id, 0);
        assert_eq!(seek_action.target, "12");
        assert_eq!(seek_action.unit, SeekUnit::RelTime);
    }
}
