use futures_util::{SinkExt, StreamExt};
use tokio::time::{timeout, Duration};
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

use crate::types::{WispContext, ConnectionType};

// used as a sort of way for the
// "frontend" to do stuff based on
// what the HTTP server responds with :3
pub struct WispHTTPResponse {
    pub status: String,
    pub headers: String,
    pub body: String,
}

impl WispHTTPResponse {
    pub fn status(&self) -> &str {
        &self.status
    }
    pub fn headers(&self) -> &str {
        &self.headers
    }
    pub fn body(&self) -> &str {
        &self.body
    }
}

// the big one!
// this'll send a HTTP request over
// the established wisp connection
// and receive a response, it'll
// return a WispHTTPResponse obj.
// ---------------------------------
// @param ws_ctx     - current WispContext
// @param http_url   - HTTP url to connect to (NO HTTPS)
// @param headers    - http headers :3
pub async fn WispHTTPRequest(
    ws_ctx: &mut WispContext,
    http_url: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<WispHTTPResponse, Box<dyn std::error::Error>> {
    let ws_stream = ws_ctx.connection.as_mut().expect("WebSocket not initialized");
    let (mut write, mut read) = ws_stream.split();

    let stream_id: u32 = rand::random();

    let parsed = Url::parse(http_url)?;
    let host = parsed.host_str().ok_or("Missing host")?;
    let port = parsed.port_or_known_default().ok_or("Missing port")?;
    let path = parsed.path();
    let query = parsed.query().map(|q| format!("?{}", q)).unwrap_or_default();
    let full_path = format!("{}{}", path, query);

    // CONNECT
    let mut connect_packet = vec![0x01];
    connect_packet.extend(&stream_id.to_le_bytes());
    connect_packet.push(ws_ctx.conn_type as u8);
    connect_packet.extend(&(port as u16).to_le_bytes());
    connect_packet.extend(host.as_bytes());
    write.send(Message::Binary(connect_packet)).await?;

    // HTTP req
    let mut http_request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n", full_path, host);
    if let Some(hdrs) = headers {
        for (k, v) in hdrs {
            http_request.push_str(&format!("{}: {}\r\n", k, v));
        }
    }
    http_request.push_str("\r\n");

    let mut data_packet = vec![0x02];
    data_packet.extend(&stream_id.to_le_bytes());
    data_packet.extend(http_request.as_bytes());
    write.send(Message::Binary(data_packet)).await?;

    let mut body = String::new();
    let mut status_line = String::new();
    let mut headers = String::new();

    // just read max 2 packets max (0x02 + optional 0x04)
    for _ in 0..2 {
        let msg = timeout(Duration::from_secs(5), read.next()).await;

        match msg {
            Ok(Some(Ok(Message::Binary(data)))) if data.len() >= 5 => {
                let packet_type = data[0];
                let sid = u32::from_le_bytes([data[1], data[2], data[3], data[4]]);

                if sid != stream_id {
                    continue;
                }

                match packet_type {
                    0x02 => {
                        let payload = &data[5..];
                        let response_str = String::from_utf8_lossy(payload);
                        body.push_str(&response_str);
                        break;
                    }
                    0x04 => {
                        break;
                    }
                    _ => {}
                }
            }
            _ => break,
        }
    }

    let mut lines = body.lines();
    if let Some(status) = lines.next() {
        status_line = status.to_string();
    }
    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        headers.push_str(line);
        headers.push('\n');
    }

    Ok(WispHTTPResponse {
        status: status_line,
        headers,
        body,
    })
}