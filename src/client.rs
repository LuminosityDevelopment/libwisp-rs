use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
use crate::WispPktType;

use crate::types::{WispContext, ConnectionType};

// open a new wisp connection
// to the target URL and use
// this in ws_ctx.
// ---------------------------------
// @param ws_ctx     - current WispContext
// @param wispURL    - Wisp server URL to connect to (e.g: 'wss://example.com/')
pub async fn WispSetServer(ws_ctx: &mut WispContext, wispURL: &str) {
    ws_ctx.server_url = wispURL.to_string();

    let (mut ws_stream, _) = connect_async(wispURL).await.unwrap();

    if let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                if data.len() >= 5 && data[0] == WispPktType::CONTINUE as u8 {
                    let stream_id = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);
                    if stream_id != 0 {
                        panic!("Expected CONTINUE with stream_id 0, got {}", stream_id);
                    }
                }
            }
            other => {
                panic!("Unexpected initial message: {:?}", other);
            }
        }
    } else {
        panic!("No initial CONTINUE packet received from Wisp server.");
    }

    ws_ctx.connection = Some(ws_stream);
}

// get the wisp server based
// on ws_ctx
// ---------------------------------
// @param ws_ctx     - current WispContext
pub fn WispGetServer(ws_ctx: &WispContext) -> String {
    ws_ctx.server_url.clone()
}

// connection types,
// 0x01 - tcp
// 0x02 - udp
// ---------------------------------
// @param ws_ctx     - current WispContext
// @param conn       - connection type (see enum ConnectionType)
pub fn WispSetConnectionType(ws_ctx: &mut WispContext, conn: ConnectionType) {
    ws_ctx.conn_type = conn;
}

// get the current connection type based
// on whats in ws_ctx
// ---------------------------------
// @param ws_ctx     - current WispContext
pub fn WispGetConnectionType(ws_ctx: &WispContext) -> ConnectionType {
    ws_ctx.conn_type
}

// close the current wisp connection
// and set a new server URL
// ---------------------------------
// @param ws_ctx     - current WispContext
// @param newWispURL - the new wisp server to connect to 
pub async fn WispSwitchServer(ws_ctx: &mut WispContext, newWispURL: &str) {
    WispClose(ws_ctx).await;
    WispSetServer(ws_ctx, newWispURL).await;
}

// no more wisp!!
// ---------------------------------
// @param ws_ctx     - current WispContext
pub async fn WispClose(ws_ctx: &mut WispContext) {
    if let Some(mut stream) = ws_ctx.connection.take() {
        let _ = stream.close(None).await;
    }
}