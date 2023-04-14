use hyperm_mqtt::coding::*;

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

#[test]
fn encode_str_writes_str_to_buffer() {
    let mut buffer = [0u8; 10];
    let str = &[0x41u8, 0xF0, 0xAA, 0x9B, 0x94];

    let result = encode_str(&mut buffer, std::str::from_utf8(str).unwrap());

    assert!(result.is_ok());
    assert_eq!(&[0x00u8, 0x05, 0x41, 0xF0, 0xAA, 0x9B, 0x94], &buffer[..7]);
}

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

#[test]
fn encode_bin_data_writes_data_to_buffer() {
    let mut buffer = [0u8; 10];
    let data = &[1, 2, 3, 4, 5][..];

    let result = encode_bin_data(&mut buffer, data);

    assert!(result.is_ok());
    assert_eq!(&buffer[..data.len() + 2], &[0, 5, 1, 2, 3, 4, 5]);
}
