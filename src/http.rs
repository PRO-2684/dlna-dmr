//! HTTP-related code.

use super::{
    DMROptions,
    xml::{av_transport::AVTransport, rendering_control::RenderingControl},
};
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};
use log::info;
use quick_xml::{DeError, escape::escape};
use std::{io::Result as IoResult, net::SocketAddrV4, str::FromStr, sync::Arc};

/// A trait for handling HTTP requests for a DLNA DMR (Digital Media Renderer).
///
/// ## Handlers
///
/// Usually, you'll only need to override [`post_rendering_control`](HTTPServer::post_rendering_control) and [`post_av_transport`](HTTPServer::post_av_transport) to implement your own handlers. Note that overriding the parent without calling the child will make it an orphan.
///
/// - GET
///     - [`get_device_spec`](HTTPServer::get_device_spec)
///     - [`get_rendering_control`](HTTPServer::get_rendering_control)
///     - [`get_av_transport`](HTTPServer::get_av_transport)
///     - [`get_ignore`](HTTPServer::get_ignore)
/// - POST
///     - [`post_device_spec`](HTTPServer::post_device_spec)
///     - [`post_rendering_control`](HTTPServer::post_rendering_control)
///     - [`post_av_transport`](HTTPServer::post_av_transport)
///     - [`post_ignore`](HTTPServer::post_ignore)
///
/// ## Other Methods
///
/// Usually you don't need to override these methods.
///
/// - Override [`run_http`](HTTPServer::run_http) if you decide to change the HTTP server backend, or for a finer control over the server's behavior.
pub trait HTTPServer: Sync {
    /// Create and run a HTTP server with the given options.
    fn run_http(&'static self, options: Arc<DMROptions>) -> impl Future<Output = IoResult<()>> + Send {async {
        let ip = options.ip;
        let http_port = options.http_port;
        let listener = tokio::net::TcpListener::bind(SocketAddrV4::new(ip, http_port)).await?;
        info!("HTTP server listening on {ip}:{http_port}");

        let app = Router::new()
            .route(
                "/DeviceSpec",
                get(async || Self::get_device_spec(options).await).post(Self::post_device_spec),
            )
            .route(
                "/RenderingControl",
                get(Self::get_rendering_control).post(async |s: String| {
                    self.post_rendering_control(RenderingControl::from_str(&s))
                        .await
                }),
            )
            .route(
                "/AVTransport",
                get(Self::get_av_transport).post(async |s: String| {
                    self.post_av_transport(AVTransport::from_str(&s)).await
                }),
            )
            .route(
                "/Ignore",
                get(Self::get_ignore).post(async || self.post_ignore().await),
            );
        // TODO: Using state to pass `self`

        axum::serve(listener, app).await
    } }

    // POST Request handlers for specific endpoints.

    /// Handles POST requests for `/DeviceSpec`.
    fn post_device_spec() -> impl Future<Output = impl IntoResponse> + Send {
        async { StatusCode::METHOD_NOT_ALLOWED }
    }

    /// Handles POST requests for `/RenderingControl`.
    #[allow(
        unused_variables,
        reason = "This is a dummy trait method, intended to be overridden"
    )]
    fn post_rendering_control(
        &self,
        rendering_control: Result<RenderingControl, DeError>,
    ) -> impl Future<Output = impl IntoResponse> + Send {
        async { StatusCode::METHOD_NOT_ALLOWED }
    }

    /// Handles POST requests for `/AVTransport`.
    #[allow(
        unused_variables,
        reason = "This is a dummy trait method, intended to be overridden"
    )]
    fn post_av_transport(
        &self,
        av_transport: Result<AVTransport, DeError>,
    ) -> impl Future<Output = impl IntoResponse> + Send {
        async { StatusCode::METHOD_NOT_ALLOWED }
    }

    /// Handles POST requests for `/Ignore`.
    fn post_ignore(&self) -> impl Future<Output = impl IntoResponse> + Send {
        async { StatusCode::NO_CONTENT }
    }

    // GET Request handlers for specific endpoints.

    /// Handles GET requests for `/DeviceSpec`.
    #[must_use]
    fn get_device_spec(options: Arc<DMROptions>) -> impl Future<Output = impl IntoResponse> + Send {
        async move {
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
            (
                StatusCode::OK,
                [("Content-Type", r#"text/xml; charset="utf-8""#)],
                xml,
            )
        }
    }

    /// Handles GET requests for `/RenderingControl`.
    #[must_use]
    fn get_rendering_control() -> impl Future<Output = impl IntoResponse> + Send {
        async {
            (
                StatusCode::OK,
                [("Content-Type", r#"text/xml; charset="utf-8""#)],
                include_str!("./template/RenderingControl.xml"),
            )
        }
    }

    /// Handles GET requests for `/AVTransport`.
    #[must_use]
    fn get_av_transport() -> impl Future<Output = impl IntoResponse> + Send {
        async {
            (
                StatusCode::OK,
                [("Content-Type", r#"text/xml; charset="utf-8""#)],
                include_str!("./template/AVTransport.xml"),
            )
        }
    }

    /// Handles GET requests for `/Ignore`.
    #[must_use]
    fn get_ignore() -> impl Future<Output = impl IntoResponse> + Send {
        async { StatusCode::NO_CONTENT }
    }
}
