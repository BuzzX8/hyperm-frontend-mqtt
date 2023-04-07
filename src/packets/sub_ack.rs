use super::*;

pub struct SubAck {
    id: PacketId,
    reason_codes: Vec<ReasonCode>,
}

#[derive(Clone)]
pub enum ReasonCode {}

impl SubAck {
    pub fn new(reason_codes: &[ReasonCode]) -> SubAck {
        SubAck {
            id: new_packet_id(),
            reason_codes: Vec::from(reason_codes),
        }
    }
}
