use std::{mem::size_of, str::Utf8Error};

use crate::packets::{
    conn_ack::ConnAck, connect::Connect, disconnect::Disconnect, pub_ack::PubAck,
    pub_comp::PubComp, pub_rec::PubRec, pub_rel::PubRel, publish::Publish, sub_ack::SubAck,
    subscribe::Subscribe,
};

type Result<T> = std::result::Result<(T, usize), DecodingError>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecodingError {
    BufferTooSmall,
    InvalidUtf8String(Utf8Error)
}

impl From<Utf8Error> for DecodingError {
    fn from(value: Utf8Error) -> Self {
        DecodingError::InvalidUtf8String(value)
    }
}

pub fn decode_u16(buffer: &[u8]) -> Result<u16> {
    const SIZE: usize = size_of::<u16>();

    if buffer.len() < SIZE {
        return Err(DecodingError::BufferTooSmall);
    }

    let mut b = [0u8; SIZE];
    b.copy_from_slice(&buffer[..SIZE]);

    Ok((u16::from_be_bytes(b), SIZE))
}

pub fn decode_u32(buffer: &[u8]) -> Result<u32> {
    const SIZE: usize = size_of::<u32>();

    if buffer.len() < SIZE {
        return Err(DecodingError::BufferTooSmall);
    }

    let mut b = [0u8; SIZE];
    b.copy_from_slice(&buffer[..SIZE]);

    Ok((u32::from_be_bytes(b), SIZE))
}

pub fn decode_str(buffer: &[u8]) -> Result<String> {
    let (len, offset) = decode_u16(&buffer)?;
    let len = len as usize;

    if buffer.len() < len + offset {
        return Err(DecodingError::BufferTooSmall);
    }

    let str = std::str::from_utf8(&buffer[offset..len])?;

    Ok((str.to_owned(), offset + len))
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
