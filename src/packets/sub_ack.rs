use super::*;

pub struct SubAck {
    id: PacketId,
    reason_codes: Vec<SubAckReasonCode>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SubAckReasonCode {
    GrantedQos0 = 0x00,
    GrantedQos1 = 0x01,
    GrantedQos2 = 0x02,
    UnspecifiedError = 0x80,
    ImplementationSpecificError = 0x83,
    NotAuthorized = 0x87,
    TopicFilterInvalid = 0x8F,
    PacketIdentifierInUse = 0x91,
    QuotaExceeded = 0x97,
    SharedSubscriptionsNotSupported = 0x9E,
    SubscriptionIdentifiersNotSupported = 0xA1,
    WildcardSubscriptionsNotSupported = 0xA2,
}

impl SubAck {
    pub fn new(reason_codes: &[SubAckReasonCode]) -> SubAck {
        SubAck {
            id: new_packet_id(),
            reason_codes: reason_codes.into(),
        }
    }

    pub fn id(&self) -> PacketId {
        self.id
    }

    pub fn reason_codes(&self) -> &[SubAckReasonCode] {
        &self.reason_codes
    }
}

#[cfg(test)]
mod tests {
    use super::sub_ack::*;

    #[test]
    fn subscribe_new_creates_packet() {
        let reason_codes = [SubAckReasonCode::GrantedQos0, SubAckReasonCode::GrantedQos1];
        let sub_ack = SubAck::new(&reason_codes[..]);

        assert_ne!(0, sub_ack.id());
        assert_eq!(&reason_codes[..], sub_ack.reason_codes());
    }
}
