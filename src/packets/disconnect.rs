use super::reason_code::ReasonCode;

pub struct Disconnect {
    reason_code: ReasonCode,
}

impl Disconnect {
    pub fn new(reason_code: ReasonCode) -> Disconnect {
        Disconnect { reason_code }
    }

    pub fn reason_code(&self) -> ReasonCode {
        self.reason_code
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::reason_code::ReasonCode;

    #[test]
    fn new_creates_correct_packet() {
        let packet = super::Disconnect::new(ReasonCode::NormalDisconnection);

        assert_eq!(ReasonCode::NormalDisconnection, packet.reason_code());
    }
}