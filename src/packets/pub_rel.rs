use super::{new_packet_id, PacketId};

pub struct PubRel {
    id: PacketId,
    reason_code: ReasonCode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReasonCode {
    Success = 0x00,
    PacketIdentifierNotFound = 0x92,
}

impl PubRel {
    pub fn new(reason_code: ReasonCode) -> PubRel {
        PubRel {
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
    fn new_creates_pub_rel_packet() {
        let reason_code = ReasonCode::PacketIdentifierNotFound;
        let pub_rel = PubRel::new(reason_code);

        assert_ne!(0, pub_rel.id());
        assert_eq!(reason_code, pub_rel.reason_code());
    }
}
