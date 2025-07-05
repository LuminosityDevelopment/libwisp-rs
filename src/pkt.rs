// src/pkt.rs
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::{SinkExt, StreamExt};
use crate::types::WispContext;
use tokio::time::{timeout, Duration};

// WispSendPkt - sends a raw binary packet over the Wisp connection
// @param ws_ctx    - current WispContext
// @param send_buf  - buffer to send
pub async fn WispSendPkt(
    ws_ctx: &mut WispContext,
    send_buf: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = ws_ctx
        .connection
        .as_mut()
        .ok_or("WebSocket not initialized")?;
    
    ws_stream.send(Message::Binary(send_buf.to_vec())).await?;
    Ok(())
}

// WispReadPkt - reads a raw binary packet from the Wisp connection
// @param ws_ctx   - current WispContext
// @param read_buf - mutable buffer to fill with received data
// @returns number of bytes written into read_buf
// WispReadPkt - reads a raw binary packet from the Wisp connection
// @param ws_ctx   - current WispContext
// @param read_buf - mutable buffer to fill with received data
// @returns number of bytes written into read_buf
pub async fn WispReadPkt(
    ws_ctx: &mut WispContext,
    read_buf: &mut Vec<u8>,
) -> Result<usize, Box<dyn std::error::Error>> {
    let ws_stream = ws_ctx
        .connection
        .as_mut()
        .ok_or("WebSocket not initialized")?;

    let message = timeout(Duration::from_secs(5), ws_stream.next()).await;

    match message {
        Ok(Some(Ok(Message::Binary(data)))) => {
            read_buf.clear();
            read_buf.extend_from_slice(&data);
            Ok(data.len())
        }
        Ok(Some(Ok(_))) => Err("Received non-binary message".into()),
        Ok(Some(Err(e))) => Err(Box::new(e)),
        Ok(None) => Err("WebSocket closed".into()),
        Err(_) => Err("Read timeout".into()),
    }
}
