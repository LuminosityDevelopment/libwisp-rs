# Struct: WispHTTPResponse

Represents an HTTP response received via the Wisp server.

### Fields

- `status: String`  
  The HTTP status line (e.g. `HTTP/1.1 200 OK`).

- `headers: String`  
  Raw HTTP response headers as a string.

- `body: String`  
  The full HTTP response body.

### Methods

- `status(&self) -> &str`  
  Returns the status line.

- `headers(&self) -> &str`  
  Returns the headers.

- `body(&self) -> &str`  
  Returns the body.

---

See also:

- [`WispHTTPRequest`](./WispHTTPRequest.md)
