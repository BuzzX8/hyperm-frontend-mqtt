pub mod connect;
pub mod conn_ack;
pub mod reason_code;
pub mod disconnect;
pub mod subscribe;
pub mod subscription_request;
pub mod sub_ack;

#[derive(Clone)]
pub enum Qos {
    Qos0,
    Qos1,
    Qos2
}

pub type PacketId = u16;

pub(crate) fn new_packet_id() -> PacketId {
    0
}