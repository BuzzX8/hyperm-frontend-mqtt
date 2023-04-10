use std::mem::size_of;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EncodingError {
    BufferTooSmall
}

pub fn encode_u16(buffer: &mut [u8], value: u16) -> Result<(), EncodingError>{
    if buffer.len() < size_of::<u16>() {
        return Err(EncodingError::BufferTooSmall);
    }
    
    buffer[0] = (value >> 8) as u8;
    buffer[1] = value as u8;

    Ok(())
}

#[cfg(test)]
mod encode_u16_tests {
    use super::{EncodingError, encode_u16};
    
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
        assert_eq!(EncodingError::BufferTooSmall, result.err().unwrap());
    }
}