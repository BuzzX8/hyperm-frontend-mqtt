use crate::packets::{
    conn_ack::ConnAck, connect::Connect, disconnect::Disconnect, pub_ack::PubAck,
    pub_comp::PubComp, pub_rec::PubRec, pub_rel::PubRel, publish::Publish, sub_ack::SubAck,
    subscribe::Subscribe,
};

type Result<T> = std::result::Result<T, DecodingError>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecodingError {
    BufferTooSmall,
}

pub fn decode_connect(buffer: &[u8]) -> Result<Connect> {
    todo!()
}

pub fn decode_conn_ack(buffer: &[u8]) -> Result<ConnAck> {
    todo!()
}

pub fn decode_disconnect(buffer: &[u8]) -> Result<Disconnect> {
    todo!()
}

pub fn decode_subscribe(buffer: &[u8]) -> Result<Subscribe> {
    todo!()
}

pub fn decode_sub_ack(buffer: &[u8]) -> Result<SubAck> {
    todo!()
}

pub fn decode_publish(buffer: &[u8]) -> Result<Publish> {
    todo!()
}

pub fn decode_pub_ack(buffer: &[u8]) -> Result<PubAck> {
    todo!()
}

pub fn decode_pub_rec(buffer: &[u8]) -> Result<PubRec> {
    todo!()
}

pub fn decode_pub_rel(buffer: &[u8]) -> Result<PubRel> {
    todo!()
}

pub fn decode_pub_comp(buffer: &[u8]) -> Result<PubComp> {
    todo!()
}
