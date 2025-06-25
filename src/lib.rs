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

pub use types::{WispContext, ConnectionType};
pub use client::{WispSetServer, WispGetServer, WispSetConnectionType, WispGetConnectionType, WispSwitchServer, WispClose};
pub use http::{WispHTTPRequest, WispHTTPResponse};