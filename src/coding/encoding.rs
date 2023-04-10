use std::mem::size_of;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EncodingError {
    BufferTooSmall,
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