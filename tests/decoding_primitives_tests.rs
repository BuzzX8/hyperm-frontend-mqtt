use std::mem::size_of;

use hyperm_mqtt::coding::decoding::{decode_u16, decode_u32, DecodingError, decode_str};

#[test]
fn decode_u16_reads_value_from_buffer() {
    let buffer = [0x54, 0x53, 0, 0, 0, 0];

    let result = decode_u16(&buffer);

    assert!(result.is_ok());

    let (value, size) = result.unwrap();

    assert_eq!(0x5453, value);
    assert_eq!(size_of::<u16>(), size);
}

#[test]
fn decode_u16_returns_error_if_buffer_too_small() {
    let buffer = [87u8];

    let result = decode_u16(&buffer);

    assert!(result.is_err());
    assert_eq!(DecodingError::BufferTooSmall, result.unwrap_err());
}

#[test]
fn decode_u32_reads_value_from_buffer() {
    let buffer = [0x12u8, 0x34, 0x56, 0x78];

    let result = decode_u32(&buffer);

    assert!(result.is_ok());

    let (value, size) = result.unwrap();

    assert_eq!(0x12345678, value);
    assert_eq!(size_of::<u32>(), size);
}

#[test]
fn decode_u32_returns_error_if_buffer_to_small() {
    let result = decode_u32(&[]);

    assert!(result.is_err());
    assert_eq!(DecodingError::BufferTooSmall, result.unwrap_err());
}

#[test]
fn decode_str_reads_string_from_buffer() {
    let buffer = [0x00u8, 0x05, b'H', b'e', b'l', b'l', b'o'];

    let result = decode_str(&buffer);

    assert!(result.is_ok());

    let (str, size) = result.unwrap();

    assert_eq!("Hello", str);
    assert_eq!(buffer.len(), size);
}

#[test]
fn decode_str_returns_error_if_buffer_too_small() {
    let result = decode_str(&[]);

    assert!(result.is_err());
    assert_eq!(DecodingError::BufferTooSmall, result.unwrap_err());
}
