use std::{mem::size_of, str::Utf8Error};

use crate::packets::*;

type DecodingResult<T> = std::result::Result<(T, usize), DecodingError>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DecodingError {
    BufferTooSmall,
    InvalidUtf8String(Utf8Error),
}

impl From<Utf8Error> for DecodingError {
    fn from(value: Utf8Error) -> Self {
        DecodingError::InvalidUtf8String(value)
    }
}

macro_rules! decode_uint {
    ($buffer:ident, $type:ty) => {
        const SIZE: usize = size_of::<$type>();

        if $buffer.len() < SIZE {
            return Err(DecodingError::BufferTooSmall);
        }

        let mut b = [0u8; SIZE];
        b.copy_from_slice(&$buffer[..SIZE]);

        return Ok((<$type>::from_be_bytes(b), SIZE));
    };
}

pub fn decode_u16(buffer: &[u8]) -> DecodingResult<u16> {
    decode_uint!(buffer, u16);
}

pub fn decode_u32(buffer: &[u8]) -> DecodingResult<u32> {
    decode_uint!(buffer, u32);
}

pub fn decode_str(buffer: &[u8]) -> DecodingResult<String> {
    let (len, offset) = decode_u16(&buffer)?;
    let size = len as usize + offset;

    if buffer.len() < size {
        return Err(DecodingError::BufferTooSmall);
    }

    let str = std::str::from_utf8(&buffer[offset..size])?;

    Ok((str.to_owned(), size))
}

pub fn decode_var_int(buffer: &[u8]) -> DecodingResult<u32> {
    if let [] = buffer {
        return Err(DecodingError::BufferTooSmall);
    }

    macro_rules! check_next_byte_and_buffer {
        ($value:ident, $next_byte:ident, $size:literal) => {
            if $next_byte < 0x80 {
                return Ok(($value, $size));
            }

            if buffer.len() < ($size + 1) {
                return Err(DecodingError::BufferTooSmall);
            }
        };
    }

    let mut value = buffer[0] as u32;
    let mut next_byte: u32;

    check_next_byte_and_buffer!(value, value, 1);

    macro_rules! update_value {
        ($mask:literal, $index:literal, $bit_num:literal) => {
            value &= $mask;
            next_byte = buffer[$index] as u32;
            value = (next_byte << $bit_num) | value;
        };
    }
    
    update_value!(0x7F, 1, 7);
    check_next_byte_and_buffer!(value, next_byte, 2);
    update_value!(0x3F_FF, 2, 14);
    check_next_byte_and_buffer!(value, next_byte, 3);
    update_value!(0x1F_FF_FF, 3, 21);

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
