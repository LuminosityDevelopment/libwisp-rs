use std::collections::HashMap;
use libwisp::{
    WispContext, WispSetServer, WispSetConnectionType, WispHTTPRequest,
    WispHTTPMethod, WispClose, ConnectionType
};
use tokio;

#[tokio::main]
async fn main() {
    let mut ctx = WispContext::new();

    // connect to sigma wispserver.dev
    WispSetServer(&mut ctx, "wss://wispserver.dev/wisp/").await;

    // this is where we add our silly headers
    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "libwisp-rs/0.2.0-beta".to_string());

    // all http methods.. maybe??
    let methods = [
        WispHTTPMethod::GET,
        WispHTTPMethod::POST,
        WispHTTPMethod::DELETE,
        WispHTTPMethod::PUT,
        WispHTTPMethod::PATCH,
        WispHTTPMethod::HEAD,
        WispHTTPMethod::OPTIONS,
        WispHTTPMethod::TRACE,
        WispHTTPMethod::CONNECT,
    ];

    for method in methods {
        println!("==> Sending {:?} request <==", method);

        let url = match method {
            WispHTTPMethod::GET => "http://httpbin.org/get",
            WispHTTPMethod::POST => "http://httpbin.org/post",
            WispHTTPMethod::DELETE => "http://httpbin.org/delete",
            WispHTTPMethod::PUT => "http://httpbin.org/put",
            WispHTTPMethod::PATCH => "http://httpbin.org/patch",
            WispHTTPMethod::HEAD => "http://httpbin.org/headers",
            WispHTTPMethod::OPTIONS => "http://httpbin.org", // just use / because theres no OPTIONS endpoint
            WispHTTPMethod::TRACE => "http://httpbin.org", // won't work, httpbin doesn't expose anythng for TRACE
            WispHTTPMethod::CONNECT => "http://httpbin.org/ip", // theres no connect endpoint for httpbin.org, so just use /ip
        };

        // probably a better way to do this
        let body = if let WispHTTPMethod::POST = method {
            Some(b"test=1".to_vec())
        } else {
            Some(vec![])
        };

        match WispHTTPRequest(&mut ctx, url, method, Some(headers.clone()), body).await {
            Ok(response) => {
                println!("Status: {}", response.status());
                println!("Headers:\n{}", response.headers());
                println!("Body:\n{}\n", response.body());
            }
            Err(e) => {
                eprintln!("Request failed: {}\n", e);
            }
        }
    }

    // close wisp connection for good practice
    WispClose(&mut ctx).await;
}