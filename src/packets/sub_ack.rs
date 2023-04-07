use super::*;

pub struct SubAck {
    id: PacketId,
    reason_codes: Vec<ReasonCode>
}

#[derive(Clone)]
pub enum ReasonCode {}

impl SubAck {
    pub fn new(reson_codes: &[ReasonCode]) -> SubAck{
        let mut codes: Vec<ReasonCode> = Vec::with_capacity(reson_codes.len());
        codes.extend_from_slice(reson_codes);

        SubAck { id: new_packet_id(), reason_codes: codes }
    }
}