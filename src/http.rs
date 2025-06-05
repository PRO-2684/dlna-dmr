//! HTTP-related code.

use super::{
    DMROptions,
    xml::{av_transport::AVTransport, rendering_control::RenderingControl},
};
use log::{debug, error, info};
use quick_xml::{DeError, escape::escape};
use std::{
    fmt::Display,
    io::{Cursor, Result as IoResult},
    net::SocketAddrV4,
    str::FromStr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
use tiny_http::{Header, Method, Request, Response as GenericResponse, Server, StatusCode};

/// Valid endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endpoint {
    DeviceSpec,
    RenderingControl,
    AVTransport,
    Ignore,
}

impl Endpoint {
    /// Try to match given string with an endpoint.
    pub fn match_str(path: &str) -> Option<Self> {
        match path {
            "/DeviceSpec" => Some(Self::DeviceSpec),
            "/RenderingControl" => Some(Self::RenderingControl),
            "/AVTransport" => Some(Self::AVTransport),
            "/Ignore" => Some(Self::Ignore),
            _ => None,
        }
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::DeviceSpec => "/DeviceSpec",
            Self::RenderingControl => "/RenderingControl",
            Self::AVTransport => "/AVTransport",
            Self::Ignore => "/Ignore",
        };
        write!(f, "{s}")
    }
}

/// Response type.
pub type Response = GenericResponse<Cursor<Vec<u8>>>;

/// A trait for handling HTTP requests for a DLNA DMR (Digital Media Renderer).
///
/// ## Hierarchy Structure of Handlers
///
/// Usually, you'll only need to override [`post_rendering_control`](HTTPServer::post_rendering_control) and [`post_av_transport`](HTTPServer::post_av_transport) to implement your own handlers. Note that overriding the parent without calling the child will make it an orphan.
///
/// - [`handle_request`](HTTPServer::handle_request)
///     - [`handle_post`](HTTPServer::handle_post)
///         - [`post_device_spec`](HTTPServer::post_device_spec)
///         - [`post_rendering_control`](HTTPServer::post_rendering_control)
///         - [`post_av_transport`](HTTPServer::post_av_transport)
///         - [`post_ignore`](HTTPServer::post_ignore)
///     - [`handle_get`](HTTPServer::handle_get)
///         - [`get_device_spec`](HTTPServer::get_device_spec)
///         - [`get_rendering_control`](HTTPServer::get_rendering_control)
///         - [`get_av_transport`](HTTPServer::get_av_transport)
///         - [`get_ignore`](HTTPServer::get_ignore)
///
/// ## Other Methods
///
/// Usually you don't need to override these methods.
///
/// - Override [`run_http`](HTTPServer::run_http) if you decided to change the HTTP server backend.
/// - Override [`content_type_xml`](HTTPServer::content_type_xml) to change the HTTP headers indicating the content is of XML type.
pub trait HTTPServer {
    /// Create and run a HTTP server with the given options and running signal, blocking current thread.
    fn run_http(&self, options: DMROptions, running: Arc<AtomicBool>) {
        let address = SocketAddrV4::new(options.ip, options.http_port);
        let server = Server::http(address).expect("Failed to create HTTP server");

        info!("HTTP server listening on {}", server.server_addr());
        while running.load(Ordering::SeqCst) {
            match server.try_recv() {
                Ok(Some(request)) => {
                    if let Err(e) = Self::handle_request(self, &options, request) {
                        error!("Error handling request: {e}");
                    }
                }
                Ok(None) => {
                    // No request received, continue to the next iteration
                    thread::sleep(Duration::from_millis(100));
                }
                Err(e) => {
                    error!("Error receiving request: {e}");
                }
            }
        }
        server.unblock(); // Unblock the server to stop it gracefully.
        info!("HTTP server stopped");
    }

    // Request handling.

    /// Handles a given request and returns a response.
    ///
    /// ## Errors
    ///
    /// Returns an error if handling or responding to the request fails.
    fn handle_request(&self, options: &DMROptions, request: Request) -> IoResult<()> {
        debug!("Received request: {request:?}");
        let method = request.method();
        let is_post = match method {
            Method::Get => false,
            Method::Post => true,
            _ => {
                return request.respond(
                    Response::from_string("Method Not Allowed").with_status_code(StatusCode(405)),
                );
            }
        };
        let Some(endpoint) = Endpoint::match_str(request.url()) else {
            return request.respond(Response::from_string("").with_status_code(StatusCode(404)));
        };
        if is_post {
            Self::handle_post(self, endpoint, options, request)
        } else {
            Self::handle_get(self, endpoint, options, request)
        }
    }

    /// Handles POST requests for valid endpoints.
    ///
    /// ## Errors
    ///
    /// Returns an error if reading the request body fails or if responding to the request fails.
    #[allow(unused_variables, reason = "Blanket implementation, might be used when overriden")]
    fn handle_post(&self, endpoint: Endpoint, options: &DMROptions, mut request: Request) -> IoResult<()> {
        let mut body = String::with_capacity(request.body_length().unwrap_or_default());
        request.as_reader().read_to_string(&mut body)?;
        debug!("POST {endpoint}\n{body}");

        let response = match endpoint {
            Endpoint::DeviceSpec => Self::post_device_spec(self),
            Endpoint::RenderingControl => {
                Self::post_rendering_control(self, RenderingControl::from_str(&body))
            }
            Endpoint::AVTransport => Self::post_av_transport(self, AVTransport::from_str(&body)),
            Endpoint::Ignore => Self::post_ignore(self),
        };
        request.respond(response)?;

        Ok(())
    }

    // POST Request handlers for specific endpoints.

    /// Handles POST requests for `/DeviceSpec`.
    fn post_device_spec(&self) -> Response {
        // Method not allowed
        Response::from_string("").with_status_code(StatusCode(405))
    }

    /// Handles POST requests for `/RenderingControl`.
    #[allow(
        unused_variables,
        reason = "This is a dummy trait method, intended to be overridden"
    )]
    fn post_rendering_control(
        &self,
        rendering_control: Result<RenderingControl, DeError>,
    ) -> Response {
        // Method not allowed
        Response::from_string("").with_status_code(StatusCode(405))
    }

    /// Handles POST requests for `/AVTransport`.
    #[allow(
        unused_variables,
        reason = "This is a dummy trait method, intended to be overridden"
    )]
    fn post_av_transport(&self, av_transport: Result<AVTransport, DeError>) -> Response {
        // Method not allowed
        Response::from_string("").with_status_code(StatusCode(405))
    }

    /// Handles POST requests for `/Ignore`.
    fn post_ignore(&self) -> Response {
        // No content
        Response::from_string("").with_status_code(StatusCode(204))
    }

    // GET Request handlers for specific endpoints.

    /// Handles POST requests for valid endpoints.
    #[allow(unused_mut, reason = "Blanket implementation, might be mutated when overriden")]
    fn handle_get(&self, endpoint: Endpoint, options: &DMROptions, mut request: Request) -> IoResult<()> {
        let response = match endpoint {
            Endpoint::DeviceSpec => Self::get_device_spec(options),
            Endpoint::RenderingControl => Self::get_rendering_control(),
            Endpoint::AVTransport => Self::get_av_transport(),
            Endpoint::Ignore => Self::get_ignore(),
        };
        request.respond(response)?;

        Ok(())
    }

    /// Handles GET requests for `/DeviceSpec`.
    #[must_use]
    fn get_device_spec(options: &DMROptions) -> Response {
        /// Escapes given field under `options`.
        macro_rules! e {
            ($i:ident) => {
                escape(&options.$i)
            };
        }
        let xml = format!(
            include_str!("./template/DeviceSpec.tmpl.xml"),
            friendlyName = e!(friendly_name),
            modelName = e!(model_name),
            modelDescription = e!(model_description),
            modelURL = e!(model_url),
            manufacturer = e!(manufacturer),
            manufacturerURL = e!(manufacturer_url),
            serialNumber = e!(serial_number),
            uuid = e!(uuid),
        );
        Response::from_string(xml).with_header(Self::content_type_xml())
    }

    /// Handles GET requests for `/RenderingControl`.
    #[must_use]
    fn get_rendering_control() -> Response {
        Response::from_string(include_str!("./template/RenderingControl.xml"))
            .with_header(Self::content_type_xml())
    }

    /// Handles GET requests for `/AVTransport`.
    #[must_use]
    fn get_av_transport() -> Response {
        Response::from_string(include_str!("./template/AVTransport.xml"))
            .with_header(Self::content_type_xml())
    }

    /// Handles GET requests for `/Ignore`.
    #[must_use]
    fn get_ignore() -> Response {
        // No content
        Response::from_string("").with_status_code(StatusCode(204))
    }

    // Helper methods.

    /// HTTP header that indicates the content type for XML responses.
    #[must_use]
    fn content_type_xml() -> Header {
        Header::from_bytes("Content-Type", br#"text/xml; charset="utf-8""#).unwrap()
    }
}
