use std::mem::size_of;

use crate::packets::{
    conn_ack::ConnAck, connect::Connect, disconnect::Disconnect, pub_ack::PubAck,
    pub_comp::PubComp, pub_rec::PubRec, pub_rel::PubRel, publish::Publish, sub_ack::SubAck,
    subscribe::Subscribe, unsub_ack::UnsubAck, unsubscribe::Unsubscribe,
};

type Result = std::result::Result<usize, EncodingError>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EncodingError {
    BufferTooSmall,
    StringTooLong,
    ValueTooBig { max_val: u32 },
}

pub fn encode_u16(buffer: &mut [u8], value: u16) -> Result {
    const SIZE: usize = size_of::<u16>();

    match buffer {
        _ if buffer.len() < SIZE => Err(EncodingError::BufferTooSmall),
        b => {
            b[..SIZE].copy_from_slice(&value.to_be_bytes()[..]);
            Ok(SIZE)
        }
    }
}

pub fn encode_u32(buffer: &mut [u8], value: u32) -> Result {
    const SIZE: usize = size_of::<u32>();
    
    match buffer {
        _ if buffer.len() < SIZE => Err(EncodingError::BufferTooSmall),
        b => {
            b[..SIZE].copy_from_slice(&value.to_be_bytes()[..]);
            Ok(SIZE)
        }
    }
}

pub fn encode_str(buffer: &mut [u8], str: &str) -> Result {
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

pub fn encode_var_int(buffer: &mut [u8], value: u32) -> Result {
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

pub fn encode_bin_data(buffer: &mut [u8], data: &[u8]) -> Result {
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

pub fn encode_connect(buffer: &mut [u8], connect: &Connect) -> Result {
    todo!()
}

pub fn encode_conn_ack(buffer: &mut [u8], conn_ack: &ConnAck) -> Result {
    todo!()
}

pub fn encode_disconnect(buffer: &mut [u8], disconnect: &Disconnect) -> Result {
    todo!()
}

pub fn encode_publish(buffer: &mut [u8], publish: &Publish) -> Result {
    todo!()
}

pub fn encode_pub_ack(buffer: &mut [u8], pub_ack: &PubAck) -> Result {
    todo!()
}

pub fn encode_pub_rec(buffer: &mut [u8], pub_rec: &PubRec) -> Result {
    todo!()
}

pub fn encode_pub_rel(buffer: &mut [u8], pub_rel: &PubRel) -> Result {
    todo!()
}

pub fn encode_pub_comp(buffer: &mut [u8], pub_comp: &PubComp) -> Result {
    todo!()
}

pub fn encode_subscribe(buffer: &mut [u8], subscribe: &Subscribe) -> Result {
    todo!()
}

pub fn encode_sub_ack(buffer: &mut [u8], sub_ack: &SubAck) -> Result {
    todo!()
}

pub fn encode_unsubscribe(buffer: &mut [u8], unsubscribe: &Unsubscribe) -> Result {
    todo!()
}

pub fn encode_unsub_ack(buffer: &mut [u8], unsub_ack: &UnsubAck) -> Result {
    todo!()
}

pub fn encode_ping_req(buffer: &mut [u8]) -> Result {
    todo!()
}

pub fn encode_ping_resp(buffer: &mut [u8]) -> Result {
    todo!()
}
