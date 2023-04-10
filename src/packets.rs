pub mod conn_ack;
pub mod connect;
pub mod disconnect;
pub mod sub_ack;
pub mod subscribe;
pub mod unsub_ack;
pub mod unsubscribe;
pub mod publish;

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
