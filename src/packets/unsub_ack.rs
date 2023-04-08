use super::*;

pub struct UnsubAck {
    id: PacketId,
    reason_code: ReasonCode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
    fn unsub_ack_new_creates_packet() {
        let reason_code = ReasonCode::Success;
        let unsub_ack = UnsubAck::new(reason_code);

        assert_ne!(0, unsub_ack.id());
        assert_eq!(reason_code, unsub_ack.reason_code());
    }
}