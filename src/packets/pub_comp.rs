use super::{new_packet_id, PacketId};

pub struct PubComp {
    id: PacketId,
    reason_code: PubCompReasonCode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PubCompReasonCode {
    Success = 0x00,
    PacketIdentifierNotFound = 0x92,
}

impl PubComp {
    pub fn new(reason_code: PubCompReasonCode) -> PubComp {
        PubComp {
            id: new_packet_id(),
            reason_code,
        }
    }

    pub fn id(&self) -> PacketId {
        self.id
    }

    pub fn reason_code(&self) -> PubCompReasonCode {
        self.reason_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_pub_comp_packet() {
        let reason_code = PubCompReasonCode::PacketIdentifierNotFound;
        let pub_rel = PubComp::new(reason_code);

        assert_ne!(0, pub_rel.id());
        assert_eq!(reason_code, pub_rel.reason_code());
    }
}
