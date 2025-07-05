# Enum: WispPktType

Defines the packet types used in the Wisp protocol.

| Variant   | Value  | Description                         |
|-----------|--------|-------------------------------------|
| `CONNECT` | `0x01` | Initiate connection to target host  |
| `DATA`    | `0x02` | Transmit application data           |
| `CONTINUE`| `0x03` | Initial "OK" from server            |
| `CLOSE`   | `0x04` | Close the stream or session         |

Used internally for constructing raw Wisp packets.

---

See also:

- [`WispSendPkt`](./WispSendPkt.md)
- [`WispReadPkt`](./WispReadPkt.md)
