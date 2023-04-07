use super::*;

pub struct UnsubAck {
    id: PacketId,
    reason_code: ReasonCode,
}

pub enum ReasonCode {
    Success = 0x00,
    NoSubscriptionExisted = 0x11,
    UnspecifiedError = 0x80,
    ImplementationSpecificError = 0x83,
    NotAuthorized = 0x87,
    TopicFilterInvalid = 0x8F,
    PacketIdentifierInUse = 0x91,
}

impl UnsubAck {
    pub fn new(reason_code: ReasonCode) -> UnsubAck {
        UnsubAck {
            id: new_packet_id(),
            reason_code,
        }
    }
}
