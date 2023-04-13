use hyperm_mqtt::{packets::connect::Connect, coding::encoding::encode_connect};

#[test]
#[ignore]
fn encode_connect_writes_packet_to_buffer() {
    let mut buffer = [0u8; 50];    
    let expected = &[
        0x10, //packet type
        0xA, //remaining length
        0u8, 0x04, b'M', b'Q', b'T', b'T', //Protocol name
        0x05, //Protocol version
        0xCE, //Connect Flags
        0x0A, //Keep Alive
        0x00, //Prop length
        0x00, 0x03, b'c', b'i', b'd', //client id        
    ];
    let connect = Connect::new("client-id");

    let result = encode_connect(&mut buffer, &connect);

    assert!(result.is_ok());
    assert_eq!(expected, &buffer[..expected.len()]);
}
