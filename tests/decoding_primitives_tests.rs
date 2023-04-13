use std::mem::size_of;

use hyperm_mqtt::coding::decoding::{decode_u16, DecodingError};

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