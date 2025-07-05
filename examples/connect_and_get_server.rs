use libwisp::{WispContext, WispSetServer, WispGetServer, WispClose};
use tokio;

#[tokio::main]
async fn main() {
    let mut ctx = WispContext::new();

    // connect to a wisp server
    WispSetServer(&mut ctx, "wss://wispserver.dev/wisp/").await;

    // print current connected server url
    let server_url = WispGetServer(&ctx);
    println!("connected to wisp server: {}", server_url);

    // close connection for good practice
    WispClose(&mut ctx).await;
}