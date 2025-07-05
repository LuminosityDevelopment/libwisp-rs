use futures_util::{SinkExt, StreamExt};
use tokio::time::{timeout, Duration};
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;
use crate::types::{WispContext, ConnectionType};
use std::fmt;
use crate::WispPktType;

// http methods :3
// example:
// WispHTTPMethod::GET
#[derive(Debug)]
pub enum WispHTTPMethod {
    GET,
    POST,
    DELETE,
    PUT,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
}

impl fmt::Display for WispHTTPMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            WispHTTPMethod::GET => "GET",
            WispHTTPMethod::POST => "POST",
            WispHTTPMethod::DELETE => "DELETE",
            WispHTTPMethod::PUT => "PUT",
            WispHTTPMethod::PATCH => "PATCH",
            WispHTTPMethod::HEAD => "HEAD",
            WispHTTPMethod::OPTIONS => "OPTIONS",
            WispHTTPMethod::TRACE => "TRACE",
            WispHTTPMethod::CONNECT => "CONNECT",
        };
        write!(f, "{}", s)
    }
}

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
    method: WispHTTPMethod,
    headers: Option<HashMap<String, String>>,
    body: Option<Vec<u8>>,
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

    // CONNECT packet
    let mut connect_packet: Vec<u8> = vec![WispPktType::CONNECT as u8];
    connect_packet.extend(&stream_id.to_le_bytes());
    connect_packet.push(ws_ctx.conn_type as u8);
    connect_packet.extend(&(port as u16).to_le_bytes());
    connect_packet.extend(host.as_bytes());
    write.send(Message::Binary(connect_packet)).await?;

    let mut http_request = format!(
        "{} {} HTTP/1.1\r\nHost: {}\r\n",
        method.to_string(),
        full_path,
        host
    );

    // i dont trust the user to do this themselves :/
    let mut merged_headers = headers.unwrap_or_default();
    if let Some(ref body_bytes) = body {
        merged_headers
            .entry("Content-Length".to_string())
            .or_insert_with(|| body_bytes.len().to_string());
    }

    for (k, v) in &merged_headers {
        http_request.push_str(&format!("{}: {}\r\n", k, v));
    }

    http_request.push_str("\r\n");

    let mut data_packet = vec![WispPktType::DATA as u8];
    data_packet.extend(&stream_id.to_le_bytes());
    data_packet.extend(http_request.as_bytes());

    if let Some(body_bytes) = body {
        data_packet.extend(&body_bytes);
    }

    write.send(Message::Binary(data_packet)).await?;

    let mut response_body = String::new();
    let mut status_line = String::new();
    let mut response_headers = String::new();

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
                    x if x == WispPktType::DATA as u8 => {
                        let payload = &data[5..];
                        let response_str = String::from_utf8_lossy(payload);
                        response_body.push_str(&response_str);
                        break;
                    }
                    x if x == WispPktType::CLOSE as u8 => {
                        break;
                    }
                    _ => {}
                }
            }
            _ => break,
        }
    }

    let mut lines = response_body.lines();
    if let Some(status) = lines.next() {
        status_line = status.to_string();
    }
    for line in lines {
        if line.trim().is_empty() {
            break;
        }
        response_headers.push_str(line);
        response_headers.push('\n');
    }

    Ok(WispHTTPResponse {
        status: status_line,
        headers: response_headers,
        body: response_body,
    })
}