use std::{mem::size_of, str::Utf8Error};

use crate::packets::{
    conn_ack::ConnAck, connect::Connect, disconnect::Disconnect, pub_ack::PubAck,
    pub_comp::PubComp, pub_rec::PubRec, pub_rel::PubRel, publish::Publish, sub_ack::SubAck,
    subscribe::Subscribe,
};

type DecodingResult<T> = std::result::Result<(T, usize), DecodingError>;

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

pub fn decode_u16(buffer: &[u8]) -> DecodingResult<u16> {
    const SIZE: usize = size_of::<u16>();

    if buffer.len() < SIZE {
        return Err(DecodingError::BufferTooSmall);
    }

    let mut b = [0u8; SIZE];
    b.copy_from_slice(&buffer[..SIZE]);

    Ok((u16::from_be_bytes(b), SIZE))
}

pub fn decode_u32(buffer: &[u8]) -> DecodingResult<u32> {
    const SIZE: usize = size_of::<u32>();

    if buffer.len() < SIZE {
        return Err(DecodingError::BufferTooSmall);
    }

    let mut b = [0u8; SIZE];
    b.copy_from_slice(&buffer[..SIZE]);

    Ok((u32::from_be_bytes(b), SIZE))
}

pub fn decode_str(buffer: &[u8]) -> DecodingResult<String> {
    let (len, offset) = decode_u16(&buffer)?;
    let len = len as usize;

    if buffer.len() < len + offset {
        return Err(DecodingError::BufferTooSmall);
    }

    let str = std::str::from_utf8(&buffer[offset..len])?;

    Ok((str.to_owned(), offset + len))
}

pub fn decode_var_int(buffer: &[u8]) -> DecodingResult<u32> {
    if let [] = buffer {
        return  Err(DecodingError::BufferTooSmall);
    }

    let mut value = buffer[0] as u32;

    if value < 0x80 {
        return Ok((value, 1));
    }

    if buffer.len() < 2 {
        return Err(DecodingError::BufferTooSmall);
    }

    value &= 0x7F;
    let mut next_byte = buffer[1] as u32;
    value = (next_byte << 7) | value;

    if next_byte < 0x80 {
        return Ok((value, 2));
    }

    if buffer.len() < 3 {
        return Err(DecodingError::BufferTooSmall);
    }

    value &= 0x3F_FF;
    next_byte = buffer[2] as u32;
    value = (next_byte << 14) | value;

    if next_byte < 0x80 {
        return Ok((value, 3));
    }

    if buffer.len() < 4 {
        return Err(DecodingError::BufferTooSmall);
    }

    value &= 0x1F_FF_FF;
    next_byte = buffer[3] as u32;
    value = (next_byte << 21) | value;

    Ok((value, 4))
}

pub fn decode_bin_data(buffer: &[u8]) -> DecodingResult<Vec<u8>> {

    let (value, offset) = decode_u16(buffer)?;
    let size = offset + value as usize;

    if buffer.len() < size {
        return Err(DecodingError::BufferTooSmall);
    }

    Ok((buffer[offset..].into(), size))
}

pub fn decode_connect(buffer: &[u8]) -> DecodingResult<Connect> {
    todo!()
}

pub fn decode_conn_ack(buffer: &[u8]) -> DecodingResult<ConnAck> {
    todo!()
}

pub fn decode_disconnect(buffer: &[u8]) -> DecodingResult<Disconnect> {
    todo!()
}

pub fn decode_subscribe(buffer: &[u8]) -> DecodingResult<Subscribe> {
    todo!()
}

pub fn decode_sub_ack(buffer: &[u8]) -> DecodingResult<SubAck> {
    todo!()
}

pub fn decode_publish(buffer: &[u8]) -> DecodingResult<Publish> {
    todo!()
}

pub fn decode_pub_ack(buffer: &[u8]) -> DecodingResult<PubAck> {
    todo!()
}

pub fn decode_pub_rec(buffer: &[u8]) -> DecodingResult<PubRec> {
    todo!()
}

pub fn decode_pub_rel(buffer: &[u8]) -> DecodingResult<PubRel> {
    todo!()
}

pub fn decode_pub_comp(buffer: &[u8]) -> DecodingResult<PubComp> {
    todo!()
}
