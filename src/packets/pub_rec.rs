use super::{new_packet_id, PacketId};

pub struct PubRec {
    id: PacketId,
    reason_code: PubRecReasonCode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PubRecReasonCode {
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

impl PubRec {
    pub fn new(reason_code: PubRecReasonCode) -> PubRec {
        PubRec {
            id: new_packet_id(),
            reason_code,
        }
    }

    pub fn id(&self) -> PacketId {
        self.id
    }

    pub fn reason_code(&self) -> PubRecReasonCode {
        self.reason_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_pub_rec_packet() {
        let reason_code = PubRecReasonCode::ImplementationSpecificError;
        let pub_ack = PubRec::new(reason_code);

        assert_ne!(0, pub_ack.id());
        assert_eq!(reason_code, pub_ack.reason_code());
    }
}
