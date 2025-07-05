# Enum: WispHTTPMethod

Represents supported HTTP request methods in the Wisp protocol.

### Variants

| Variant   | Description                     |
|-----------|---------------------------------|
| `GET`     | HTTP GET request                |
| `POST`    | HTTP POST request               |
| `DELETE`  | HTTP DELETE request             |
| `PUT`     | HTTP PUT request                |
| `PATCH`   | HTTP PATCH request              |
| `HEAD`    | HTTP HEAD request               |
| `OPTIONS` | HTTP OPTIONS request            |
| `TRACE`   | HTTP TRACE request              |
| `CONNECT` | HTTP CONNECT request            |

Use this enum to specify the HTTP method when calling [`WispHTTPRequest`](./WispHTTPRequest.md).

---

See also:

- [`WispHTTPRequest`](./WispHTTPRequest.md)