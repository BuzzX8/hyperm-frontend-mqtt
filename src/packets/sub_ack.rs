use super::*;

pub struct SubAck {
    id: PacketId,
    reason_codes: Vec<ReasonCode>,
}

#[derive(Clone)]
pub enum ReasonCode {
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
    WildcardSubscriptionsNotSupported = 0xA2
}

impl SubAck {
    pub fn new(reason_codes: &[ReasonCode]) -> SubAck {
        SubAck {
            id: new_packet_id(),
            reason_codes: reason_codes.into(),
        }
    }

    pub fn id(&self) -> PacketId {
        self.id
    }

    pub fn reason_codes(&self) -> &[ReasonCode] {
        &self.reason_codes
    }
}
