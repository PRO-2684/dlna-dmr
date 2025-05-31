//! Module for deserializing and extracting information from `RenderingControl` XML messages.
//!
//! Documentation on `RenderingControl` v1 can be found [here](http://upnp.org/specs/av/UPnP-av-RenderingControl-v1-Service.pdf).

use quick_xml::{DeError, de};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

/// The envelope structure for `RenderingControl` XML messages.
///
/// Usually, once deserialized, you'll call [`RenderingControlEnvelope::into_inner`] to consume it and get the actual content of the message, which you could match against the [`RenderingControl`] enum to determine the specific action type. For an even simpler usage, [`RenderingControl`] implements `FromStr`, allowing you to directly deserialize from a XML envelope string.
///
/// ## Example
///
/// ```rust
/// use quick_xml::de::from_str;
/// use dlna_dmr::xml::rendering_control::{Channel, RenderingControlEnvelope, RenderingControl};
///
/// let xml = r#"<?xml version="1.0" ?>
/// <s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/" s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">
///     <s:Body>
///         <u:SetVolume xmlns:u="urn:schemas-upnp-org:service:RenderingControl:1">
///             <DesiredVolume>50</DesiredVolume>
///             <Channel>Master</Channel>
///             <InstanceID>0</InstanceID>
///         </u:SetVolume>
///     </s:Body>
/// </s:Envelope>"#;
/// let deserialized: RenderingControlEnvelope = from_str(xml).expect("Failed to deserialize XML");
/// let set_action = match deserialized.into_inner() {
///     RenderingControl::SetVolume(set) => set,
///     _ => panic!("Expected SetVolume variant"),
/// };
/// assert_eq!(set_action.instance_id, 0);
/// assert_eq!(set_action.channel, Channel::Master);
/// assert_eq!(set_action.desired_volume, 50);
/// ```
#[allow(missing_docs, reason = "Wrapper struct")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RenderingControlEnvelope {
    #[serde(rename = "@encodingStyle")]
    pub s_encoding_style: String,
    #[serde(rename = "@xmlns:s")]
    pub xmlns_s: String,
    #[serde(rename = "Body")]
    pub s_body: SBody,
}

impl RenderingControlEnvelope {
    /// Take ownership of the [`RenderingControl`] value contained in the envelope, consuming the envelope.
    #[must_use]
    pub fn into_inner(self) -> RenderingControl {
        self.s_body.content
    }
}

/// Container structure.
#[allow(missing_docs, reason = "Wrapper struct")]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SBody {
    #[serde(rename = "$value")]
    content: RenderingControl,
}

/// Different types of action that can be invoked in the `RenderingControl` service. Can be directly parsed from an XML envelope string, IGNORING the outer envelope structure.
///
/// ## Example
///
/// ```rust
/// use dlna_dmr::xml::rendering_control::{Channel, RenderingControl};
///
/// let xml = r#"<?xml version="1.0" ?>
/// <s:Envelope xmlns:s="http://schemas.xmlsoap.org/soap/envelope/" s:encodingStyle="http://schemas.xmlsoap.org/soap/encoding/">
///     <s:Body>
///         <u:SetVolume xmlns:u="urn:schemas-upnp-org:service:RenderingControl:1">
///             <DesiredVolume>50</DesiredVolume>
///             <Channel>Master</Channel>
///             <InstanceID>0</InstanceID>
///         </u:SetVolume>
///     </s:Body>
/// </s:Envelope>"#;
/// let rendering_control: RenderingControl = xml.parse().expect("Failed to parse RenderingControl");
/// let RenderingControl::SetVolume(set_action) = rendering_control else {
///     panic!("Expected SetVolume variant");
/// };
/// assert_eq!(set_action.instance_id, 0);
/// assert_eq!(set_action.channel, Channel::Master);
/// assert_eq!(set_action.desired_volume, 50);
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum RenderingControl {
    /// Returns a list of the currently defined presets.
    ListPresets(ListPresets),
    /// Restores (a subset) of the state variables to the values associated with the specified preset.
    SelectPreset(SelectPreset),
    /// Retrieves the current value of the Mute setting of the channel for the specified instance of this service.
    GetMute(GetMute),
    /// Sets the Mute state variable of the specified instance of this service to the specified value.
    SetMute(SetMute),
    /// Retrieves the current value of the Volume state variable of the specified channel for the specified instance of this service.
    GetVolume(GetVolume),
    /// Sets the Volume state variable of the specified Instance and Channel to the specified value.
    SetVolume(SetVolume),
}

impl FromStr for RenderingControl {
    type Err = DeError;
    /// Deserialize from an envelope, IGNORING the outer envelope structure.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let envelope: RenderingControlEnvelope = de::from_str(s)?;
        Ok(envelope.into_inner())
    }
}

/// Arguments for [`RenderingControl::ListPresets`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ListPresets {
    /// The XML namespace for the `AVTransport` service.
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    /// The virtual instance of the `AVTransport` service to which the action applies.
    #[serde(rename = "InstanceID")]
    pub instance_id: u32,
}

/// Arguments for [`RenderingControl::SelectPreset`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SelectPreset {
    /// The XML namespace for the `AVTransport` service.
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    /// Specify the name of a device preset.
    #[serde(rename = "PresetName")]
    pub preset_name: PresetName,
    /// The virtual instance of the `AVTransport` service to which the action applies.
    #[serde(rename = "InstanceID")]
    pub instance_id: u32,
}

/// Possible values for the [`preset_name`](SelectPreset::preset_name) field of [`RenderingControl::SelectPreset`].
///
/// Currently, only [`FactoryDefaults`](PresetName::FactoryDefaults) is supported.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum PresetName {
    /// The factory settings defined by the device's manufacturer.
    FactoryDefaults,
}

impl Display for PresetName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FactoryDefaults => write!(f, "FactoryDefaults"),
        }
    }
}

/// Arguments for [`RenderingControl::GetMute`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetMute {
    /// The XML namespace for the `AVTransport` service.
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    /// A particular channel of an audio output stream.
    #[serde(rename = "Channel")]
    pub channel: Channel,
    /// The virtual instance of the `AVTransport` service to which the action applies.
    #[serde(rename = "InstanceID")]
    pub instance_id: u32,
}

/// Possible values for channels in `GetMute`, `SetMute`, `GetVolume`, and `SetVolume` actions.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// The Master channel is a logical channel and, therefore, has no spatial position associated with it.
    Master,
    // TODO: Other channels?
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Master => write!(f, "Master"),
        }
    }
}

/// Arguments for [`RenderingControl::SetMute`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetMute {
    /// The XML namespace for the `AVTransport` service.
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    /// Desired Mute state.
    #[serde(rename = "DesiredMute")]
    pub desired_mute: bool,
    /// A particular channel of an audio output stream.
    #[serde(rename = "Channel")]
    pub channel: Channel,
    /// The virtual instance of the `AVTransport` service to which the action applies.
    #[serde(rename = "InstanceID")]
    pub instance_id: u32,
}

/// Arguments for [`RenderingControl::GetVolume`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GetVolume {
    /// The XML namespace for the `AVTransport` service.
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    /// A particular channel of an audio output stream.
    #[serde(rename = "Channel")]
    pub channel: Channel,
    /// The virtual instance of the `AVTransport` service to which the action applies.
    #[serde(rename = "InstanceID")]
    pub instance_id: u32,
}

/// Arguments for [`RenderingControl::SetVolume`].
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SetVolume {
    /// The XML namespace for the `AVTransport` service.
    #[serde(rename = "@xmlns:u")]
    pub xmlns_u: String,
    /// Desired volume level. Should be between 0 and 100, inclusive.
    #[serde(rename = "DesiredVolume")]
    pub desired_volume: u16,
    /// A particular channel of an audio output stream.
    #[serde(rename = "Channel")]
    pub channel: Channel,
    /// The virtual instance of the `AVTransport` service to which the action applies.
    #[serde(rename = "InstanceID")]
    pub instance_id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    fn get_xml(path: &str) -> RenderingControl {
        let xml = read_to_string(format!("tests/RenderingControl/{path}"))
            .expect("Failed to read XML file");
        let av_transport: RenderingControl = xml.parse().expect("Failed to parse AVTransport");
        av_transport
    }

    #[test]
    fn test_list_presets() {
        let list_presets = get_xml("ListPresets.xml");
        let RenderingControl::ListPresets(list) = list_presets else {
            panic!("Expected ListPresets variant");
        };
        assert_eq!(list.instance_id, 0);
    }

    #[test]
    fn test_select_preset() {
        let select_preset = get_xml("SelectPreset.xml");
        let RenderingControl::SelectPreset(select) = select_preset else {
            panic!("Expected SelectPreset variant");
        };
        assert_eq!(select.instance_id, 0);
        assert_eq!(select.preset_name, PresetName::FactoryDefaults);
    }

    #[test]
    fn test_get_mute() {
        let get_mute = get_xml("GetMute.xml");
        let RenderingControl::GetMute(get) = get_mute else {
            panic!("Expected GetMute variant");
        };
        assert_eq!(get.instance_id, 0);
        assert_eq!(get.channel, Channel::Master);
    }

    #[test]
    fn test_set_mute() {
        let set_mute = get_xml("SetMute.xml");
        let RenderingControl::SetMute(set) = set_mute else {
            panic!("Expected SetMute variant");
        };
        assert_eq!(set.instance_id, 0);
        assert_eq!(set.channel, Channel::Master);
        assert!(set.desired_mute);
    }

    #[test]
    fn test_get_volume() {
        let get_volume = get_xml("GetVolume.xml");
        let RenderingControl::GetVolume(get) = get_volume else {
            panic!("Expected GetVolume variant");
        };
        assert_eq!(get.instance_id, 0);
        assert_eq!(get.channel, Channel::Master);
    }

    #[test]
    fn test_set_volume() {
        let set_volume = get_xml("SetVolume.xml");
        let RenderingControl::SetVolume(set) = set_volume else {
            panic!("Expected SetVolume variant");
        };
        assert_eq!(set.instance_id, 0);
        assert_eq!(set.channel, Channel::Master);
        assert_eq!(set.desired_volume, 50);
    }
}
