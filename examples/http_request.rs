use std::collections::HashMap;
use libwisp::{WispContext, WispSetServer, WispHTTPRequest, WispClose};
use tokio;

#[tokio::main]
async fn main() {
    let mut ctx = WispContext::new();

    // connect to the wisp server first
    WispSetServer(&mut ctx, "wss://wisp.mercurywork.shop/").await;

    // prepare optional headers
    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "libwisp-rs/0.1.0".to_string());

    // make an http request over wisp!
    match WispHTTPRequest(&mut ctx, "http://example.com", Some(headers)).await {
        Ok(response) => {
            println!("status: {}", response.status());
            println!("headers:\n{}", response.headers());
            println!("body:\n{}", response.body());
        }
        Err(e) => {
            eprintln!("request failed: {}", e);
        }
    }

    // close connection for good practice
    WispClose(&mut ctx).await;
}