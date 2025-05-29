//! HTTP-related code.

use std::{io::{Cursor, Result}, net::SocketAddrV4, sync::{atomic::{AtomicBool, Ordering}, Arc}, thread};
use super::DMROptions;
use log::{error, info};
use tiny_http::{Method, Request, Response as GenericResponse, Server, StatusCode};

type Response = GenericResponse<Cursor<Vec<u8>>>;

/// A simple HTTP server for handling DLNA DMR related requests.
pub struct HTTPServer {
    server: Server,
    options: DMROptions,
    running: Arc<AtomicBool>,
}

impl HTTPServer {
    /// Creates a new HTTP server with the given options.
    pub fn new(options: DMROptions, running: Arc<AtomicBool>) -> Self {
        let address = SocketAddrV4::new(*options.address.ip(), options.http_port);
        let server = Server::http(address).expect("Failed to create HTTP server");
        HTTPServer { server, options, running }
    }

    /// Runs the HTTP server, listening for requests.
    pub fn run(&self) {
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
        if let Err(e) = self.stop() {
            error!("Error stopping HTTP server: {e}");
        } else {
            info!("HTTP server stopped");
        }
    }

    /// Stops the HTTP server.
    fn stop(&self) -> Result<()> {
        self.server.unblock();
        Ok(())
    }

    /// Handles a given request and returns a response.
    fn handle_request(&self, request: Request) -> Result<()> {
        let method = request.method();
        let is_post = match method {
            Method::Get => false,
            Method::Post => true,
            _ => {
                return request.respond(
                    Response::from_string("Method Not Allowed").with_status_code(StatusCode(405)),
                );
            },
        };
        let path = request.url();
        let response = match path {
            // Posting to valid endpoints
            "/DeviceSpec" | "/RenderingControl" | "/AVTransport" | "/Ignore" if is_post => Self::post_invalid(),
            // Handle GET requests for valid endpoints
            "/DeviceSpec" => Self::get_device_spec(),
            "/RenderingControl" => Self::get_rendering_control(),
            "/AVTransport" => Self::get_av_transport(),
            "/Ignore" => Self::get_ignore(),
            // Handle invalid paths
            _ => Self::not_found(),
        };
        request.respond(response)
    }

    /// Handles POST requests for valid endpoints.
    fn post_invalid() -> Response {
        // Error for now
        Response::from_string("Invalid InstanceID").with_status_code(StatusCode(718))
    }

    /// Handles GET requests for `/DeviceSpec`.
    fn get_device_spec() -> Response {
        Response::from_string("TODO: DeviceSpec response")
    }

    /// Handles GET requests for `/RenderingControl`.
    fn get_rendering_control() -> Response {
        Response::from_string("TODO: RenderingControl response")
    }

    /// Handles GET requests for `/AVTransport`.
    fn get_av_transport() -> Response {
        Response::from_string("TODO: AVTransport response")
    }

    /// Handles GET requests for `/Ignore`.
    fn get_ignore() -> Response {
        Response::from_string("TODO: Ignore response")
    }

    /// Handles other requests (requests to invalid endpoints)
    fn not_found() -> Response {
        Response::from_string("Not Found").with_status_code(StatusCode(404))
    }
}

