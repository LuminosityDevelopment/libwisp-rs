use libwisp::{WispContext, WispSetServer, WispSwitchServer, WispClose};
use tokio;

#[tokio::main]
async fn main() {
    let mut ctx = WispContext::new();

    // connect to first wisp server
    WispSetServer(&mut ctx, "wss://wispserver.dev/wisp/").await;
    println!("connected to first server (wss://wisp.mercurywork.shop)");

    // switch to a second wisp server
    WispSwitchServer(&mut ctx, "wss://wispserver.dev/adblocking/").await;
    println!("switched to second server (wss://wispserver.dev/adblocking/)");

    // close connection for good practice
    WispClose(&mut ctx).await;
}