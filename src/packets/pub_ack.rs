use super::{new_packet_id, PacketId};

pub struct PubAck {
    id: PacketId,
    reason_code: ReasonCode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReasonCode {
    Success = 0x00,
    NoMatchingSubscribers = 0x10,
    UnspecifiedError = 0x80,
    ImplementationSpecificError = 0x83,
    NotAuthorized = 0x87,
    TopicNameInvalid = 0x90,
    PacketIdentifierInUse = 0x91,
    QuotaExceeded = 0x97,
    PayloadFormatInvalid = 0x99
}

impl PubAck {
    pub fn new(reason_code: ReasonCode) -> PubAck {
        PubAck {
            id: new_packet_id(),
            reason_code,
        }
    }

    pub fn id(&self) -> PacketId {
        self.id
    }

    pub fn reason_code(&self) -> ReasonCode {
        self.reason_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_pub_ack_packet() {
        let reason_code = ReasonCode::ImplementationSpecificError;
        let pub_ack = PubAck::new(reason_code);

        assert_ne!(0, pub_ack.id());
        assert_eq!(reason_code, pub_ack.reason_code());
    }
}
