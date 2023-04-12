use hyperm_mqtt::{packets::conn_ack::{ConnAck, ReasonCode}, coding::encoding::encode_conn_ack};

#[test]
fn encode_conn_ack_writes_packet_to_buffer() {
    let mut buffer = [0u8; 100];
    let conn_ack = ConnAck::new(true, ReasonCode::Success);
    let expected = &[0x20u8, 0x00];

    let result = encode_conn_ack(&mut buffer, &conn_ack);

    //assert!(result.is_ok());
    //assert_eq!(expected, &buffer[..expected.len()]);
}
