# Enum: ConnectionType

Defines the type of underlying connection protocol to use.

| Variant | Value | Description           |
|---------|--------|----------------------|
| `Tcp`   | `0x01` | TCP connection (default) |
| `Udp`   | `0x02` | UDP connection (untested) |

> **Note:** UDP support is currently untested.

---

See also:

- [`WispSetConnectionType`](./WispSetConnectionType.md) — set connection type in context
- [`WispGetConnectionType`](./WispGetConnectionType.md) — get connection type from context
- [`WispContext`](./WispContext.md) — context holding connection info
