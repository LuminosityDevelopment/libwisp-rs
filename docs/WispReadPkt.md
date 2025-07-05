# Async Function: WispReadPkt

Reads a raw binary packet from the Wisp connection into a buffer.

- `ws_ctx` - mutable reference to the current [`WispContext`](./WispContext.md).
- `read_buf` - mutable byte buffer to store the received data.

**Returns:**  
`Result<usize, Box<dyn std::error::Error>>` — number of bytes read into the buffer.

---

See also:

- [`WispSendPkt`](./WispSendPkt.md)
- [`WispContext`](./WispContext.md)
