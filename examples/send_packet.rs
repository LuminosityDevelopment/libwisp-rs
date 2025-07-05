use libwisp::{
    WispContext, WispSetServer, WispSetConnectionType, WispClose,
    ConnectionType, WispPktType, WispSendPkt, WispReadPkt
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = WispContext::new();

    WispSetServer(&mut ctx, "wss://wispserver.dev/wisp/").await;
    WispSetConnectionType(&mut ctx, ConnectionType::Tcp);

    let stream_id: u32 = rand::random();

    // CONNECT packet!!
    let mut connect_packet = Vec::new();
    connect_packet.push(WispPktType::CONNECT as u8); // connection type (CONNECT)
    connect_packet.extend(&stream_id.to_le_bytes()); // stream id
    connect_packet.push(ctx.conn_type as u8); // connection type (default: tcp)
    connect_packet.extend(&(80u16).to_le_bytes()); // port 80 as a uint16_t
    connect_packet.extend("example.com".as_bytes()); // target http server is example.com
    WispSendPkt(&mut ctx, &connect_packet).await?;

    // HTTP GET packet!!!!
let request = b"GET / HTTP/1.1\r\n\
Host: example.com\r\n\
Connection: close\r\n\
\r\n"; // yes this is what an HTTP request looks like deal with it :sob:
    
    let mut data_packet = Vec::new();
    data_packet.push(WispPktType::DATA as u8); // connection type (DATA)
    data_packet.extend(&stream_id.to_le_bytes()); // stream id from before
    data_packet.extend(request); // HTTP request
    WispSendPkt(&mut ctx, &data_packet).await?;

    // read response from wisp srv :3
    let mut buffer = Vec::new();
    match WispReadPkt(&mut ctx, &mut buffer).await {
        Ok(size) => {
            println!("Received {} bytes", size);
            println!("{}", String::from_utf8_lossy(&buffer));
        }
        Err(e) => {
            println!("Error reading response: {}", e);
        }
    }

    // close connection for good practice !!
    WispClose(&mut ctx).await;
    Ok(())
}
