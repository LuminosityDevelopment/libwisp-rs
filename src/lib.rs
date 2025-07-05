#![allow(non_snake_case)]
#![allow(unused_imports)]
// ^^ for some reason,
//    client.rs throws
//    warning about unused
//    imports but when i remove
//    them it complains about not
//    having said imports :/

pub mod types;
pub mod client;
pub mod http;
pub mod pkt;

pub use pkt::{WispSendPkt, WispReadPkt};
pub use types::{WispContext, ConnectionType, WispPktType};
pub use client::{WispSetServer, WispGetServer, WispSetConnectionType, WispGetConnectionType, WispSwitchServer, WispClose};
pub use http::{WispHTTPRequest, WispHTTPResponse, WispHTTPMethod};