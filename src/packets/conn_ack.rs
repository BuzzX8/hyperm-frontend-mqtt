pub struct ConnAck {
    session_present: bool,
    reason_code: ReasonCode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ReasonCode {
    Success = 0x00,
    UnspecifiedError = 0x80,
    MalformedPacket = 0x81,
    ProtocolError = 0x82,
    ImplementationSpecificError = 0x83,
    UnsupportedProtocolVersion = 0x84,
    ClientIdentifierNotValid = 0x85,
    BadUserNameOrPassword = 0x86,
    NotAuthorized = 0x87,
    ServerUnavailable = 0x88,
    ServerBusy = 0x89,
    BadAuthenticationMethod = 0x8C,
    TopicNameInvalid = 0x90,
    PacketTooLarge = 0x95,
    QuotaExceeded = 0x97,
    PayloadFormatInvalid = 0x99,
    RetainNotSupported = 0x9A,
    QosNotSupported = 0x9B,
    UseAnotherServer = 0x9C,
    ServerMoved = 0x9D,
    ConnectionRateExceeded = 0x9F,
}

impl ConnAck {
    pub fn new(session_present: bool, reason_code: ReasonCode) -> ConnAck {
        ConnAck {
            session_present,
            reason_code,
        }
    }

    pub fn session_present(&self) -> bool {
        self.session_present
    }

    pub fn reason_code(&self) -> ReasonCode {
        self.reason_code
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_correct_conn_ack() {
        let reason_code = ReasonCode::BadAuthenticationMethod;
        let conn_ack = ConnAck::new(true, reason_code);

        assert!(conn_ack.session_present());
        assert_eq!(reason_code, conn_ack.reason_code());
    }
}
