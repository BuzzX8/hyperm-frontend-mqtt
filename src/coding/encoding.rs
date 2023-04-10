use std::mem::size_of;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EncodingError {
    BufferTooSmall,
    StringTooLong
}

pub fn encode_u16(buffer: &mut [u8], value: u16) -> Result<(), EncodingError> {
    if buffer.len() < size_of::<u16>() {
        return Err(EncodingError::BufferTooSmall);
    }

    buffer[0] = (value >> 8) as u8;
    buffer[1] = value as u8;

    Ok(())
}

pub fn encode_u32(buffer: &mut [u8], value: u32) -> Result<(), EncodingError> {
    if buffer.len() < size_of::<u32>() {
        return Err(EncodingError::BufferTooSmall);
    }

    buffer[0] = (value >> 24) as u8;
    buffer[1] = (value >> 16) as u8;
    buffer[2] = (value >> 8) as u8;
    buffer[3] = value as u8;

    Ok(())
}

pub fn encode_str(buffer: &mut [u8], str: &str) -> Result<(), EncodingError> {
    let str = str.as_bytes();

    if str.len() > u16::MAX as usize{
        return Err(EncodingError::StringTooLong);
    }

    if str.len() + size_of::<u16>() > buffer.len() {
        return  Err(EncodingError::BufferTooSmall);
    }

    let len = str.len() as u16;

    encode_u16(buffer, len)?;
    let buffer = &mut buffer[2..];
    buffer[..str.len()].copy_from_slice(str);

    Ok(())
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
