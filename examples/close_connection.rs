use libwisp::{WispContext, WispSetServer, WispClose};
use tokio;

#[tokio::main]
async fn main() {
    let mut ctx = WispContext::new();

    // connect first
    WispSetServer(&mut ctx, "wss://wisp.mercurywork.shop").await;
    println!("connected.");

    // close connection for good practice
    WispClose(&mut ctx).await;
    println!("connection closed.");
}