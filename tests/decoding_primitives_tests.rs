use std::mem::size_of;

use hyperm_mqtt::coding::decoding::{decode_str, decode_u16, decode_u32, DecodingError, decode_var_int, decode_bin_data};

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

#[test]
fn decode_var_int_reads_value_from_buffer() {
    let test_cases = [
        (0x7Fu32, &[0x7Fu8][..], 1),
        (128, &[0x80, 0x01], 2),
        (16_383, &[0xFF, 0x7F], 2),
        (16_384, &[0x80, 0x80, 0x01], 3),
        (2_097_151, &[0xFF, 0xFF, 0x7F], 3),
        (2_097_152, &[0x80, 0x80, 0x80, 0x01], 4),
        (268_435_455, &[0xFF, 0xFF, 0xFF, 0x7F], 4),
    ];

    for (v, b, s) in test_cases {
        decode_var_int_reads_value_from_buffer_case(b, v, s);
    }
}

fn decode_var_int_reads_value_from_buffer_case(
    buffer: &[u8],
    expected_value: u32,
    expected_size: usize,
) {
    let result = decode_var_int(buffer);

    assert!(result.is_ok());

    let (value, size) = result.unwrap();

    assert_eq!(expected_value, value);
    assert_eq!(expected_size, size);
}

#[test]
fn decode_bin_data_reads_data_from_buffer(){
    let buffer = [0u8, 5, 0x01, 0x02, 0x30, 0x04, 0x50];

    let result = decode_bin_data(&buffer);

    assert!(result.is_ok());

    let (data, size) = result.unwrap();

    assert_eq!(&buffer[2..], &data);
    assert_eq!(buffer.len(), size);
}
