mod connect;
mod conn_ack;
mod disconnect;
mod subscribe;
mod sub_ack;
mod unsubscribe;
mod unsub_ack;
mod publish;
mod pub_ack;
mod pub_rec;
mod pub_rel;
mod pub_comp;

pub use connect::*;
pub use conn_ack::*;
pub use disconnect::*;
pub use subscribe::*;
pub use sub_ack::*;
pub use unsubscribe::*;
pub use unsub_ack::*;
pub use publish::*;
pub use pub_ack::*;
pub use pub_rec::*;
pub use pub_rel::*;
pub use pub_comp::*;

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Qos {
    Qos0,
    Qos1,
    Qos2,
}

pub type PacketId = u16;

pub(super) fn new_packet_id() -> PacketId {
    let time_span = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    time_span.as_nanos() as u16
}
