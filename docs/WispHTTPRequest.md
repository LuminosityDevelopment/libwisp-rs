# Async Function: WispHTTPRequest

Sends an HTTP GET request over the Wisp connection and collects the response.

- `ws_ctx` - mutable reference to the active [`WispContext`](./WispContext.md) (must have an open connection).
- `http_url` - the HTTP URL to request (only HTTP, no HTTPS).
- `headers` - optional HTTP headers as a map of key-value strings.

**Returns:**  
A [`WispHTTPResponse`](./WispHTTPResponse.md) struct containing the status line, headers, and body.

### Details:

- Generates a random `stream_id` for multiplexing requests.
- Parses the URL to extract host, port, and path.
- Sends a Wisp CONNECT packet, followed by the HTTP GET request.
- Reads binary response packets, aggregating the HTTP response.
- Stops reading once the response is complete.

---

See also:

- [`WispContext`](./WispContext.md)
- [`WispHTTPResponse`](./WispHTTPResponse.md)
- [`ConnectionType`](./ConnectionType.md)
