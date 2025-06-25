# Struct: WispContext

Holds the runtime context and configuration for a Wisp connection.

### Fields

- `server_url: String`  
  The URL of the Wisp server currently connected to.

- `conn_type: ConnectionType`  
  The connection protocol to use ([`ConnectionType`](./ConnectionType.md)).

- `connection: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>`  
  The underlying WebSocket connection to the Wisp server.

### Usage

Create a new default context with `WispContext::new()`.  
Then configure and open a connection using functions such as [`WispSetServer`](./WispSetServer.md).

---

## Function: `WispContext::new() -> WispContext`

Creates a new default `WispContext` with empty server URL, TCP connection type, and no active connection.

---

See also:

- [`ConnectionType`](./ConnectionType.md)
- [`WispSetServer`](./WispSetServer.md)
- [`WispHTTPRequest`](./WispHTTPRequest.md)
