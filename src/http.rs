//! HTTP-related code.

use super::{DMROptions, extract};
use log::{debug, error, info};
use quick_xml::escape::escape;
use std::{
    io::{Cursor, Result},
    net::SocketAddrV4,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};
use tiny_http::{Header, Method, Request, Response as GenericResponse, Server, StatusCode};

type Response = GenericResponse<Cursor<Vec<u8>>>;

/// A simple HTTP server for handling DLNA DMR related requests.
pub struct HTTPServer {
    server: Server,
    options: DMROptions,
    running: Arc<AtomicBool>,
}

impl HTTPServer {
    // Create, run, and stop the HTTP server.

    /// Creates a new HTTP server with the given options.
    pub fn new(options: DMROptions, running: Arc<AtomicBool>) -> Self {
        let address = SocketAddrV4::new(*options.address.ip(), options.http_port);
        let server = Server::http(address).expect("Failed to create HTTP server");
        Self {
            server,
            options,
            running,
        }
    }

    /// Runs the HTTP server, listening for requests.
    pub fn run(&self) {
        info!("HTTP server listening on {}", self.server.server_addr());
        while self.running.load(Ordering::SeqCst) {
            match self.server.try_recv() {
                Ok(Some(request)) => {
                    if let Err(e) = self.handle_request(request) {
                        error!("Error handling request: {e}");
                    }
                }
                Ok(None) => {
                    // No request received, continue to the next iteration
                    thread::yield_now();
                }
                Err(e) => {
                    error!("Error receiving request: {e}");
                }
            }
        }
        self.server.unblock(); // Unblock the server to stop it gracefully.
        info!("HTTP server stopped");
    }

    // Request handling methods.

    /// Handles a given request and returns a response.
    fn handle_request(&self, request: Request) -> Result<()> {
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
        let path = request.url();
        let response = match path {
            // Posting to valid endpoints
            "/DeviceSpec" | "/RenderingControl" | "/AVTransport" | "/Ignore" if is_post => {
                Self::post_all(request)?;
                return Ok(());
            }
            // Handle GET requests for valid endpoints
            "/DeviceSpec" => self.get_device_spec(),
            "/RenderingControl" => Self::get_rendering_control(),
            "/AVTransport" => Self::get_av_transport(),
            "/Ignore" => Self::get_ignore(),
            // Handle invalid paths
            _ => Self::not_found(),
        };
        // debug!("Responding to request: {response:?}");
        request.respond(response)
    }

    /// Handles POST requests for all valid endpoints.
    fn post_all(mut request: Request) -> Result<()> {
        let mut body = String::with_capacity(request.body_length().unwrap_or_default());
        request.as_reader().read_to_string(&mut body)?;
        let path = request.url();
        for text in extract(path, &body) {
            info!("{text}");
        }

        debug!("POST {path}\n{body}");

        let response =
            Response::from_string("Invalid InstanceID").with_status_code(StatusCode(718));
        request.respond(response)?;

        Ok(())
    }

    /// Handles GET requests for `/DeviceSpec`.
    fn get_device_spec(&self) -> Response {
        /// Escapes given field under `self.options`.
        macro_rules! e {
            ($i:ident) => {
                escape(&self.options.$i)
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
    fn get_rendering_control() -> Response {
        Response::from_string(include_str!("./template/RenderingControl.xml"))
            .with_header(Self::content_type_xml())
    }

    /// Handles GET requests for `/AVTransport`.
    fn get_av_transport() -> Response {
        Response::from_string(include_str!("./template/AVTransport.xml"))
            .with_header(Self::content_type_xml())
    }

    /// Handles GET requests for `/Ignore`.
    fn get_ignore() -> Response {
        Response::from_string("").with_status_code(StatusCode(204))
    }

    /// Handles other requests (requests to invalid endpoints)
    fn not_found() -> Response {
        Response::from_string("Not Found").with_status_code(StatusCode(404))
    }

    /// HTTP header that indicates the content type for XML responses.
    fn content_type_xml() -> Header {
        Header::from_bytes("Content-Type", br#"text/xml; charset="utf-8""#).unwrap()
    }
}
