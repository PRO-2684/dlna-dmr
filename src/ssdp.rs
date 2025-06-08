//! SSDP-related code.

use log::{error, info, trace};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use std::{
    io::{Error, ErrorKind, Result},
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Duration,
};
use tokio::{net::UdpSocket, time::sleep};

/// A SSDP server implementation.
#[derive(Debug)]
pub struct SSDPServer {
    socket: UdpSocket,
    address: SocketAddrV4,
    uuid: String,
    http_port: u16,
}

impl SSDPServer {
    /// The multicast address used for SSDP discovery.
    const SSDP_MULTICAST_ADDR: SocketAddrV4 =
        SocketAddrV4::new(Ipv4Addr::new(239, 255, 255, 250), 1900);
    /// The SSDP server's name.
    const SSDP_SERVER_NAME: &'static str = "CustomSSDP/1.0";
    // /// The timeout for reading from the socket in milliseconds.
    // const SOCKET_READ_TIMEOUT: u64 = 1000;
    /// Interval for sending keep-alive messages.
    const KEEP_ALIVE_INTERVAL: Duration = Duration::from_secs(60);

    /// Creates a new SSDP server bound to the specified address with the given UUID and HTTP port.
    pub async fn new(address: SocketAddrV4, uuid: String, http_port: u16) -> Result<Self> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        socket.set_nonblocking(true)?;
        socket.set_reuse_address(true)?;
        socket.bind(&SockAddr::from(SocketAddrV4::new(
            Ipv4Addr::UNSPECIFIED,
            address.port(),
        )))?;
        // socket.set_read_timeout(Some(Duration::from_millis(Self::SOCKET_READ_TIMEOUT)))?; // FIXME: Do we need this?
        // Set the socket to allow broadcast.
        socket.set_broadcast(true)?;
        // Join the SSDP multicast group.
        socket.join_multicast_v4(
            Self::SSDP_MULTICAST_ADDR.ip(), // Multicast address
            address.ip(),                   // Use the unspecified address for the local interface
        )?;
        // Convert the socket to a Tokio UdpSocket.
        let socket = UdpSocket::from_std(socket.into())?;

        Ok(Self {
            socket,
            address,
            uuid,
            http_port,
        })
    }

    /// Send a SSDP notify message with given Notification Type, Notification Sub Type and Unique Service Name.
    ///
    /// ## Arguments
    ///
    /// - `nt`: Notification Type
    /// - `nts`: Notification Sub Type
    /// - `usn`: Unique Service Name
    async fn notify(&self, nt: &str, nts: &str, usn: &str) -> Result<()> {
        let message = format!(
            "NOTIFY * HTTP/1.1\r\n\
             HOST: {}\r\n\
             NT: {}\r\n\
             NTS: {}\r\n\
             USN: {}\r\n\
             LOCATION: http://{}/description.xml\r\n\
             CACHE-CONTROL: max-age=1800\r\n\
             SERVER: {}\r\n\
             \r\n",
            Self::SSDP_MULTICAST_ADDR,
            nt,
            nts,
            usn,
            self.address,
            Self::SSDP_SERVER_NAME
        );
        self.socket
            .send_to(message.as_bytes(), &Self::SSDP_MULTICAST_ADDR)
            .await?;
        Ok(())
    }

    /// Broadcast a notify message for given `service` with given Notification Sub Type.
    async fn notify_service(&self, service: &str, nts: &str) -> Result<()> {
        self.notify(
            &format!("urn:schemas-upnp-org:service:{service}:1"),
            nts,
            &format!(
                "uuid:{uuid}::urn:schemas-upnp-org:service:{service}:1",
                uuid = self.uuid
            ),
        )
        .await
    }

    /// Broadcast multiple relevant notify messages with given Notification Sub Type.
    async fn notify_all(&self, nts: &str) -> Result<()> {
        let uuid_with_prefix = format!("uuid:{}", self.uuid);

        self.notify(
            "upnp:rootdevice",
            nts,
            &format!("{uuid_with_prefix}::upnp:rootdevice"),
        )
        .await?;
        self.notify(&uuid_with_prefix, nts, &uuid_with_prefix)
            .await?;
        for service in ["RenderingControl", "AVTransport", "ConnectionManager"] {
            self.notify_service(service, nts).await?;
        }

        Ok(())
    }

    /// Broadcast multiple relevant `ssdp:alive` messages.
    async fn alive(&self) -> Result<()> {
        self.notify_all("ssdp:alive").await
    }

    /// Broadcast multiple relevant `ssdp:alive` messages periodically, blocking current thread. (Keep-alive / Heartbeat)
    pub async fn keep_alive(&self) {
        info!("Starting SSDP keep-alive thread");
        loop {
            if let Err(e) = self.alive().await {
                error!("Failed to send SSDP alive message: {e}");
            } else {
                trace!("SSDP alive message sent");
            }
            sleep(Self::KEEP_ALIVE_INTERVAL).await;
        }
    }

    /// Broadcast multiple relevant `ssdp:byebye` messages.
    async fn byebye(&self) -> Result<()> {
        self.notify_all("ssdp:byebye").await
    }

    /// Answer a SSDP message from given address.
    async fn answer(&self, address: SocketAddrV4, message: &str) -> Result<()> {
        if message.starts_with("M-SEARCH") {
            self.answer_search(address, message).await
        } else if message.starts_with("NOTIFY") {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                format!("Received unknown SSDP message: {message}"),
            ))
        }
    }

    /// Answer a M-SEARCH request.
    async fn answer_search(&self, address: SocketAddrV4, _message: &str) -> Result<()> {
        // TODO: Check if we should respond to this M-SEARCH request.
        let response = format!(
            "HTTP/1.1 200 OK\r\n\
             ST: upnp:rootdevice\r\n\
             USN: uuid:{}::upnp:rootdevice\r\n\
             Location: http://{}:{}/DeviceSpec\r\n\
             OPT: \"http://schemas.upnp.org/upnp/1/0/\"; ns=01\r\n\
             Cache-Control: max-age=900\r\n\
             Server: {}\r\n\
             EXT:\r\n\
             Date: {}\r\n\
            \r\n",
            self.uuid,
            self.address.ip(),
            self.http_port,
            Self::SSDP_SERVER_NAME,
            chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT")
        );
        trace!("Sending SSDP response to {address}: {response}");
        self.socket.send_to(response.as_bytes(), address).await?;

        Ok(())
    }

    /// Starts the SSDP server, listening for incoming messages, stops when [`running`](Self::running) is set to false, blocking current thread.
    pub async fn run(&self) {
        info!("SSDP server running on {}", self.address);

        let mut buf = [0u8; 4096];
        loop {
            match self.socket.recv_from(&mut buf).await {
                Ok((size, addr)) => {
                    let message = String::from_utf8_lossy(&buf[..size]);
                    let SocketAddr::V4(ipv4) = addr else {
                        error!("Received non-IPv4 address: {addr:?}");
                        continue;
                    };
                    trace!("Received SSDP message from {ipv4}: {message}");
                    if let Err(e) = self.answer(ipv4, &message).await {
                        error!("Error answering SSDP message: {e}");
                    }
                }
                // FIXME: Do we need this?
                Err(e) if e.kind() == ErrorKind::WouldBlock => {} // Non-blocking mode, just do nothing.
                Err(e) => {
                    error!("Error receiving SSDP message: {e}");
                }
            }
        }
    }

    /// Stops the SSDP server.
    pub async fn stop(&self) {
        if let Err(e) = self.byebye().await {
            error!("Failed to send SSDP byebye message: {e}");
        } else {
            info!("SSDP server stopped");
        }
    }
}
