//! Pretty print POST bodies.

use regex::Regex;
use std::sync::LazyLock;

/// For URI like `<CurrentURI>https://my-secret.com</CurrentURI>` in `/AVTransport` endpoint.
static CURRENT_URI_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<CurrentURI>(?P<uri>[^<]+)</CurrentURI>").unwrap());
/// For URI like `<NextURI>https://my-secret.com</NextURI>` in `/AVTransport` endpoint.
static NEXT_URI_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<NextURI>(?P<uri>[^<]+)</NextURI>").unwrap());

/// Extracts potentially useful information from given text.
pub fn extract(path: &str, text: &str) -> Vec<String> {
    match path {
        "/AVTransport" => {
            let mut result = Vec::new();
            if let Some(uri_captures) = CURRENT_URI_REGEX.captures(text) {
                if let Some(uri) = uri_captures.name("uri") {
                    result.push(format!("Current URI: {}", uri.as_str().trim()));
                }
            }
            if let Some(uri_captures) = NEXT_URI_REGEX.captures(text) {
                if let Some(uri) = uri_captures.name("uri") {
                    result.push(format!("Next URI: {}", uri.as_str().trim()));
                }
            }
            result
        }
        _ => Vec::new(),
    }
}
