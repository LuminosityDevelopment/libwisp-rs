# Async Function: WispSendPkt

Sends a raw binary packet over the Wisp connection.

- `ws_ctx` - mutable reference to the current [`WispContext`](./WispContext.md).
- `send_buf` - byte slice containing the packet to send.

**Returns:**  
`Result<(), Box<dyn std::error::Error>>`

---

See also:

- [`WispReadPkt`](./WispReadPkt.md)
- [`WispContext`](./WispContext.md)
