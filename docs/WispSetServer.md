# Async Function: WispSetServer

Connects to the specified Wisp server URL via WebSocket and stores the connection in `ws_ctx`.

- `ws_ctx` - mutable reference to the current [`WispContext`](./WispContext.md).
- `wispURL` - the Wisp server URL (e.g. `"wss://example.com/"`).

**Notes:**

- Upon connection, waits for an initial "CONTINUE" packet (type `0x03`) with stream ID 0.
- Panics if no initial packet or unexpected data is received.

---

See also:

- [`WispContext`](./WispContext.md)
- [`WispClose`](./WispClose.md)
- [`WispSwitchServer`](./WispSwitchServer.md)