//! Default values for [`DMROptions`](super::DMROptions).

use std::net::{Ipv4Addr, IpAddr};
use local_ip_address::local_ip;

/// Default IP, determined by the local machine's IP address.
pub fn ip() -> Ipv4Addr {
    let ip = local_ip().expect("Failed to get local IP address");
    match ip {
        IpAddr::V4(ip) => ip,
        IpAddr::V6(_) => panic!("IPv6 is not supported"),
    }
}

/// Default SSDP server port.
pub fn ssdp_port() -> u16 {
    1900
}

/// Default HTTP server port.
pub fn http_port() -> u16 {
    8080
}

/// Default UUID of the DMR instance, generated randomly.
pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Default friendly name of the DMR instance.
pub fn friendly_name() -> String {
    "Dummy Renderer".to_string()
}

/// Default model name of the DMR instance.
pub fn model_name() -> String {
    "Dummy Model".to_string()
}

/// Default model description of the DMR instance.
pub fn model_description() -> String {
    "A dummy DLNA DMR".to_string()
}

/// Default model URL of the DMR instance.
pub fn model_url() -> String {
    "http://example.com/dummy_model".to_string()
}

/// Default manufacturer of the DMR instance.
pub fn manufacturer() -> String {
    "Dummy Manufacturer".to_string()
}

/// Default manufacturer URL of the DMR instance.
pub fn manufacturer_url() -> String {
    "http://example.com/manufacturer".to_string()
}

/// Default serial number of the DMR instance.
pub fn serial_number() -> String {
    "12345678-1234-5678-1234-567812345678".to_string()
}
