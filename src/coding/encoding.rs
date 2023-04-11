use std::mem::size_of;

use crate::packets::{
    conn_ack::ConnAck, connect::Connect, disconnect::Disconnect, pub_ack::PubAck,
    pub_comp::PubComp, pub_rec::PubRec, pub_rel::PubRel, publish::Publish, sub_ack::SubAck,
    subscribe::Subscribe, unsub_ack::UnsubAck, unsubscribe::Unsubscribe,
};

type Result = std::result::Result<(), EncodingError>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EncodingError {
    BufferTooSmall,
    StringTooLong,
    ValueTooBig { max_val: u32 },
}

pub fn encode_u16(buffer: &mut [u8], value: u16) -> Result {
    if buffer.len() < size_of::<u16>() {
        return Err(EncodingError::BufferTooSmall);
    }

    buffer[0] = (value >> 8) as u8;
    buffer[1] = value as u8;

    Ok(())
}

pub fn encode_u32(buffer: &mut [u8], value: u32) -> Result {
    if buffer.len() < size_of::<u32>() {
        return Err(EncodingError::BufferTooSmall);
    }

    buffer[0] = (value >> 24) as u8;
    buffer[1] = (value >> 16) as u8;
    buffer[2] = (value >> 8) as u8;
    buffer[3] = value as u8;

    Ok(())
}

pub fn encode_str(buffer: &mut [u8], str: &str) -> Result {
    let str = str.as_bytes();

    if str.len() > u16::MAX as usize {
        return Err(EncodingError::StringTooLong);
    }

    if str.len() + size_of::<u16>() > buffer.len() {
        return Err(EncodingError::BufferTooSmall);
    }

    let len = str.len() as u16;

    encode_u16(buffer, len)?;
    let buffer = &mut buffer[2..];
    buffer[..str.len()].copy_from_slice(str);

    Ok(())
}

pub fn encode_var_int(buffer: &mut [u8], value: u32) -> Result {
    match buffer {
        b if value < 0x80 && b.len() >= 1 => {
            b[0] = value as u8;
        }
        b if value < 0x4000 && b.len() >= 2 => {
            b[0] = ((value & 0x7F) | 0x80) as u8;
            b[1] = (value >> 7) as u8;
        }
        b if value < 0x20_00_00 && b.len() >= 3 => {
            b[0] = ((value & 0x7F) | 0x80) as u8;
            b[1] = ((value >> 7) | 0x80) as u8;
            b[2] = (value >> 14) as u8;
        }
        b if value <= 0xFFF_FFFF && b.len() >= 4 => {
            b[0] = ((value & 0x7F) | 0x80) as u8;
            b[1] = ((value >> 7) | 0x80) as u8;
            b[2] = ((value >> 14) | 0x80) as u8;
            b[3] = (value >> 21) as u8;
        }
        _ if value > 0xFFF_FFFF => {
            return Err(EncodingError::ValueTooBig {
                max_val: 0xFFF_FFFF,
            })
        }
        _ => return Err(EncodingError::BufferTooSmall),
    }

    Ok(())
}

pub fn encode_bin_data(buffer: &mut [u8], data: &[u8]) -> Result {
    if data.len() > u16::MAX as usize {
        return Err(EncodingError::StringTooLong);
    }

    if buffer.len() < data.len() + 2 {
        return Err(EncodingError::BufferTooSmall);
    }

    encode_u16(buffer, data.len() as u16)?;

    buffer[2..data.len() + 2].copy_from_slice(data);

    Ok(())
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

#[cfg(test)]
mod encode_u16_tests {
    use super::{encode_u16, EncodingError};

    #[test]
    fn encode_u16_wites_value_to_buffer() {
        let mut buffer = [0u8; 4];
        let value = 0x1234u16;

        let result = encode_u16(&mut buffer, value);

        assert!(result.is_ok());
        assert_eq!(&[0x12, 0x34], &buffer[..=1]);
    }

    #[test]
    fn encode_u16_return_error_if_buffer_too_small() {
        let mut buffer = [0u8; 1];
        let value = 0x1234u16;

        let result = encode_u16(&mut buffer, value);

        assert!(result.is_err());
        assert_eq!(EncodingError::BufferTooSmall, result.unwrap_err());
    }
}

#[cfg(test)]
mod encode_u32_tests {
    use super::*;

    #[test]
    fn encode_u32_writes_value_to_buffer() {
        let mut buffer = [0u8; 6];
        let value = 0x12345678u32;

        let result = encode_u32(&mut buffer, value);

        assert!(result.is_ok());
        assert_eq!(&[0x12, 0x34, 0x56, 0x78], &buffer[..=3]);
    }

    #[test]
    fn encode_u32_returns_error_if_buffer_too_small() {
        let mut buffer = [0u8; 2];
        let value = 0x12345678u32;

        let result = encode_u32(&mut buffer, value);

        assert!(result.is_err());
        assert_eq!(EncodingError::BufferTooSmall, result.unwrap_err());
    }
}

#[cfg(test)]
mod encode_str_tests {
    use super::*;

    #[test]
    fn encode_str_writes_str_to_buffer() {
        let mut buffer = [0u8; 10];
        let str = &[0x41u8, 0xF0, 0xAA, 0x9B, 0x94];

        let result = encode_str(&mut buffer, std::str::from_utf8(str).unwrap());

        assert!(result.is_ok());
        assert_eq!(&[0x00u8, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94], &buffer[..7]);
    }
}

#[cfg(test)]
mod encode_var_int_tests {
    use super::*;

    #[test]
    fn encode_var_int_writes_to_buffer() {
        let test_cases = [
            (0x7Fu32, &[0x7Fu8][..]),
            (128, &[0x80, 0x01]),
            (16_383, &[0xFF, 0x7F]),
            (16_384, &[0x80, 0x80, 0x01]),
            (2_097_151, &[0xFF, 0xFF, 0x7F]),
            (2_097_152, &[0x80, 0x80, 0x80, 0x01]),
            (268_435_455, &[0xFF, 0xFF, 0xFF, 0x7F]),
        ];

        for (val, expected) in test_cases {
            encode_var_int_writes_to_buffer_case(val, expected);
        }
    }

    fn encode_var_int_writes_to_buffer_case(value: u32, expected: &[u8]) {
        let mut buffer = [0u8; 10];

        let result = encode_var_int(&mut buffer, value);

        assert!(result.is_ok());
        assert_eq!(expected, &buffer[..expected.len()]);
    }
}

#[cfg(test)]
mod encode_bin_data_tests {
    use super::*;

    #[test]
    fn encode_bin_data_writes_data_to_buffer() {
        let mut buffer = [0u8; 10];
        let data = &[1, 2, 3, 4, 5][..];

        let result = encode_bin_data(&mut buffer, data);

        assert!(result.is_ok());
        assert_eq!(&buffer[..data.len() + 2], &[0, 5, 1, 2, 3, 4, 5]);
    }
}

#[cfg(test)]
mod encode_connect_tests {
    use super::*;
    use crate::packets::connect::*;

    #[test]
    fn encode_connect_writes_packet_to_buffer() {
        let mut buffer = [0u8; 50];
        let expected = &[
            0u8, 0x04, b'M', b'Q', b'T', b'T', //Protocol name
            0x05, //Protocol version
            0xCE, //Connect Flags
            0x0A, //Keep Alive
            0x00, //
        ];
        let connect = Connect::new("client-id");

        let result = encode_connect(&mut buffer, &connect);

        assert!(result.is_ok());
        assert_eq!(expected, &buffer[..expected.len()]);
    }
}

#[cfg(test)]
mod encode_conn_ack_tests {
    use super::*;
    use crate::packets::conn_ack::*;

    #[test]
    fn encode_conn_ack_writes_packet_to_buffer() {
        let mut buffer = [0u8; 100];
        let conn_ack = ConnAck::new(true, ReasonCode::Success);
        let expected = &[0x20u8, 0x00];

        let result = encode_conn_ack(&mut buffer, &conn_ack);

        assert!(result.is_ok());
        assert_eq!(expected, &buffer[..expected.len()]);
    }
}
