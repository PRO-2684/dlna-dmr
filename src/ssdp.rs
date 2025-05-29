//! SSDP-related code.

use std::{io::{Error, ErrorKind, Result}, mem::MaybeUninit, net::{Ipv4Addr, SocketAddrV4}, sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};
use socket2::{Socket, SockAddr, Domain, Type, Protocol};
use log::{debug, error, info, trace};

/// A SSDP server implementation.
#[derive(Debug)]
pub struct SSDPServer {
    socket: Socket,
    address: SocketAddrV4,
    uuid: String,
    http_port: u16,
    running: Arc<AtomicBool>,
}

impl SSDPServer {
    /// The multicast address used for SSDP discovery.
    const SSDP_MULTICAST_ADDR: SocketAddrV4 = SocketAddrV4::new(
        Ipv4Addr::new(239, 255, 255, 250),
        1900,
    );
    /// The SSDP server's name.
    const SSDP_SERVER_NAME: &'static str = "CustomSSDP/1.0";
    /// The timeout for reading from the socket in milliseconds.
    const SOCKET_READ_TIMEOUT: u64 = 1000;

    /// Creates a new SSDP server bound to the specified address with the given UUID and HTTP port.
    pub fn new(address: SocketAddrV4, uuid: String, http_port: u16, running: Arc<AtomicBool>) -> Result<Self> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
        socket.set_nonblocking(true)?;
        socket.set_reuse_address(true)?;
        socket.bind(&SockAddr::from(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, address.port())))?;
        socket.set_read_timeout(Some(Duration::from_millis(Self::SOCKET_READ_TIMEOUT)))?;
        // Set the socket to allow broadcast.
        socket.set_broadcast(true)?;
        // Join the SSDP multicast group.
        socket.join_multicast_v4(
            &Self::SSDP_MULTICAST_ADDR.ip(), // Multicast address
            address.ip(), // Use the unspecified address for the local interface
        )?;
        Ok(SSDPServer { socket, address, uuid, http_port, running })
    }

    /// Send a SSDP notify message with given Notification Type, Notification Sub Type and Unique Service Name.
    ///
    /// ## Arguments
    ///
    /// - `nt`: Notification Type
    /// - `nts`: Notification Sub Type
    /// - `usn`: Unique Service Name
    fn notify(&self, nt: &str, nts: &str, usn: &str) -> Result<()> {
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
            Self::SSDP_MULTICAST_ADDR, nt, nts, usn,
            self.address, Self::SSDP_SERVER_NAME
        );
        self.socket.send_to(message.as_bytes(), &SockAddr::from(Self::SSDP_MULTICAST_ADDR))?;
        Ok(())
    }

    /// Broadcast a notify message for given `service` with given Notification Sub Type.
    fn notify_service(&self, service: &str, nts: &str) -> Result<()> {
        self.notify(&format!("urn:schemas-upnp-org:service:{service}:1"), nts, &format!("uuid:{uuid}::urn:schemas-upnp-org:service:{service}:1", uuid = self.uuid))
    }

    /// Broadcast multiple relevant notify messages with given Notification Sub Type.
    fn notify_all(&self, nts: &str) -> Result<()> {
        let uuid_with_prefix = format!("uuid:{}", self.uuid);

        self.notify("upnp:rootdevice", nts, &format!("{uuid_with_prefix}::upnp:rootdevice"))?;
        self.notify(&uuid_with_prefix, nts, &uuid_with_prefix)?;
        for service in ["RenderingControl", "AVTransport", "ConnectionManager"] {
            self.notify_service(service, nts)?;
        }

        Ok(())
    }

    /// Broadcast multiple relevant `ssdp:alive` messages.
    pub fn alive(&self) -> Result<()> {
        self.notify_all("ssdp:alive")
    }

    /// Broadcast multiple relevant `ssdp:byebye` messages.
    fn byebye(&self) -> Result<()> {
        self.notify_all("ssdp:byebye")
    }

    /// Answer a SSDP message from given address.
    fn answer(&self, address: SocketAddrV4, message: &str) -> Result<()> {
        if message.starts_with("M-SEARCH") {
            self.answer_search(address, message)
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
    fn answer_search(&self, address: SocketAddrV4, _message: &str) -> Result<()> {
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
            chrono::Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string()
        );
        trace!("Sending SSDP response to {address}: {response}");
        self.socket.send_to(response.as_bytes(), &SockAddr::from(address))?;

        Ok(())
    }

    /// Starts the SSDP server, listening for incoming messages, stops when [`running`](Self::running) is set to false.
    pub fn run(&self) {
        info!("SSDP server running on {}", self.address);

        let mut buf = [MaybeUninit::zeroed(); 4096];
        while self.running.load(Ordering::SeqCst) {
            match self.socket.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    // Safety: We already initialized the buffer with `MaybeUninit::zeroed()`, so we can safely assume the contents are valid.
                    let slice = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, size) };
                    let message = String::from_utf8_lossy(slice);
                    let Some(ipv4) = addr.as_socket_ipv4() else {
                        error!("Received non-IPv4 address: {addr:?}");
                        continue;
                    };
                    trace!("Received SSDP message from {ipv4}: {message}");
                    if let Err(e) = self.answer(ipv4, &message) {
                        error!("Error answering SSDP message: {e}");
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    // Non-blocking mode, just continue
                    continue;
                }
                Err(e) => {
                    error!("Error receiving SSDP message: {e}");
                }
            }
        }

        if let Err(e) = self.byebye() {
            error!("Failed to send SSDP byebye message: {e}");
        } else {
            info!("SSDP server stopped");
        }
    }
}
