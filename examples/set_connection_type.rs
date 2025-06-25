use libwisp::{WispContext, WispSetConnectionType, WispGetConnectionType, ConnectionType, WispClose};
use tokio;

#[tokio::main]
async fn main() {
    let mut ctx = WispContext::new();

    // set connection type to udp (untested)
    WispSetConnectionType(&mut ctx, ConnectionType::Udp);

    // verify connection type
    let conn_type = WispGetConnectionType(&ctx);
    println!("current connection type: {:?}", conn_type);

    // close connection for good practice (not needed here as we never connect, but.. good practice!)
    WispClose(&mut ctx).await;
}
