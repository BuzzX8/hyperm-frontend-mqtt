use std::mem::size_of;

use crate::packets::*;

type EncodingResult = std::result::Result<usize, EncodingError>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EncodingError {
    BufferTooSmall,
    StringTooLong,
    ValueTooBig { max_val: u32 },
}

macro_rules! encode_uint {
    ($buffer:ident, $value:ident, $type:ty) => {
        const SIZE: usize = size_of::<$type>();

        if $buffer.len() < SIZE {
            return Err(EncodingError::BufferTooSmall);
        }

        $buffer[..SIZE].copy_from_slice(&$value.to_be_bytes()[..]);
        return Ok(SIZE);
    };
}

pub fn encode_u8(buffer: &mut [u8], value: u8) -> EncodingResult {
    encode_uint!(buffer, value, u8);
}

pub fn encode_u16(buffer: &mut [u8], value: u16) -> EncodingResult {
    encode_uint!(buffer, value, u16);
}

pub fn encode_u32(buffer: &mut [u8], value: u32) -> EncodingResult {
    encode_uint!(buffer, value, u32);
}

pub fn encode_str(buffer: &mut [u8], str: &str) -> EncodingResult {
    match str.as_bytes() {
        _ if str.len() > u16::MAX as usize => Err(EncodingError::StringTooLong),
        _ if str.len() + size_of::<u16>() > buffer.len() => Err(EncodingError::BufferTooSmall),
        s => {
            let offset = encode_u16(buffer, str.len() as u16)?;
            let buffer = &mut buffer[offset..];
            buffer[..str.len()].copy_from_slice(s);
            Ok(offset + str.len())
        }
    }
}

pub fn encode_var_int(buffer: &mut [u8], value: u32) -> EncodingResult {
    match buffer {
        b if value < 0x80 && b.len() >= 1 => {
            b[0] = value as u8;
            Ok(1)
        }
        b if value < 0x4000 && b.len() >= 2 => {
            b[0] = ((value & 0x7F) | 0x80) as u8;
            b[1] = (value >> 7) as u8;
            Ok(2)
        }
        b if value < 0x20_00_00 && b.len() >= 3 => {
            b[0] = ((value & 0x7F) | 0x80) as u8;
            b[1] = ((value >> 7) | 0x80) as u8;
            b[2] = (value >> 14) as u8;
            Ok(3)
        }
        b if value <= 0xFFF_FFFF && b.len() >= 4 => {
            b[0] = ((value & 0x7F) | 0x80) as u8;
            b[1] = ((value >> 7) | 0x80) as u8;
            b[2] = ((value >> 14) | 0x80) as u8;
            b[3] = (value >> 21) as u8;
            Ok(4)
        }
        _ if value > 0xFFF_FFFF => Err(EncodingError::ValueTooBig {
            max_val: 0xFFF_FFFF,
        }),
        _ => Err(EncodingError::BufferTooSmall),
    }
}

pub fn encode_bin_data(buffer: &mut [u8], data: &[u8]) -> EncodingResult {
    match buffer {
        _ if data.len() > u16::MAX as usize => Err(EncodingError::StringTooLong),
        _ if buffer.len() < data.len() + 2 => Err(EncodingError::BufferTooSmall),
        b => {
            let offset = encode_u16(b, data.len() as u16)?;
            b[offset..data.len() + offset].copy_from_slice(data);
            Ok(offset + data.len())
        }
    }
}

pub fn encode_connect(buffer: &mut [u8], connect: &Connect) -> EncodingResult {
    todo!()
}

pub fn encode_conn_ack(buffer: &mut [u8], conn_ack: &ConnAck) -> EncodingResult {
    let mut offset = encode_u8(buffer, 0x20)?;
    
    offset += encode_var_int(&mut buffer[offset..], 2)?;
    offset += encode_u8(&mut buffer[offset..], conn_ack.session_present().into())?;
    offset += encode_u8(&mut buffer[offset..], conn_ack.reason_code() as u8)?;

    Ok(offset)
}

pub fn encode_disconnect(buffer: &mut [u8], disconnect: &Disconnect) -> EncodingResult {
    todo!()
}

pub fn encode_publish(buffer: &mut [u8], publish: &Publish) -> EncodingResult {
    todo!()
}

pub fn encode_pub_ack(buffer: &mut [u8], pub_ack: &PubAck) -> EncodingResult {
    todo!()
}

pub fn encode_pub_rec(buffer: &mut [u8], pub_rec: &PubRec) -> EncodingResult {
    todo!()
}

pub fn encode_pub_rel(buffer: &mut [u8], pub_rel: &PubRel) -> EncodingResult {
    todo!()
}

pub fn encode_pub_comp(buffer: &mut [u8], pub_comp: &PubComp) -> EncodingResult {
    todo!()
}

pub fn encode_subscribe(buffer: &mut [u8], subscribe: &Subscribe) -> EncodingResult {
    todo!()
}

pub fn encode_sub_ack(buffer: &mut [u8], sub_ack: &SubAck) -> EncodingResult {
    todo!()
}

pub fn encode_unsubscribe(buffer: &mut [u8], unsubscribe: &Unsubscribe) -> EncodingResult {
    todo!()
}

pub fn encode_unsub_ack(buffer: &mut [u8], unsub_ack: &UnsubAck) -> EncodingResult {
    todo!()
}

pub fn encode_ping_req(buffer: &mut [u8]) -> EncodingResult {
    todo!()
}

pub fn encode_ping_resp(buffer: &mut [u8]) -> EncodingResult {
    todo!()
}
