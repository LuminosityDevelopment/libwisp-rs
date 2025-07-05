## WispHTTPRequest

Semds an HTTP request over an existing Wisp connection.

### Parameters

- `ws_ctx: &mut WispContext` — Mutable reference to active WispContext with an open connection.
- `http_url: &str` — The HTTP URL to request (note: only supports HTTP, not HTTPS).
- `method: WispHTTPMethod` — HTTP method to use.
- `headers: Option<HashMap<String, String>>` — Optional HTTP headers.
- `body: Option<Vec<u8>>` — Optional HTTP request body.

### Returns
A [`WispHTTPResponse`](./WispHTTPResponse.md) struct containing the status line, headers, and body.

### Behavior

- Generates a random `stream_id` for request multiplexing.
- Parses the URL to extract host, port, path, and query.
- Sends a Wisp `CONNECT` packet to establish the stream.
- Sends the HTTP request line and headers, then optional body.
- Reads binary packets from the Wisp connection, reconstructing the HTTP response.
- Stops reading after the response is fully received or a timeout occurs.

---

## See also

- [`WispContext`](./WispContext.md)
- [`ConnectionType`](./ConnectionType.md)
- [`WispHTTPMethod`](#wisphhttpmethod)
- [`WispHTTPResponse`](#wisphhttpresponse)