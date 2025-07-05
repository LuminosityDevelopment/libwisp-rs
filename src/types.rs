// connection type!!
// udp is untested
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectionType {
    Tcp = 0x01,
    Udp = 0x02,
}

// our wisp context, used as
// a runtime config
pub struct WispContext {
    pub server_url: String,
    pub conn_type: ConnectionType,
    pub connection: Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
}

impl WispContext {
    pub fn new() -> Self {
        Self {
            server_url: String::new(),
            conn_type: ConnectionType::Tcp,
            connection: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum WispPktType {
    CONNECT = 0x01,
    DATA = 0x02,
    CONTINUE = 0x03,
    CLOSE = 0x04,
}