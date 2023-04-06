use super::reason_code::ReasonCode;

pub struct ConnAck {
    session_present: bool,
    reason_code: ReasonCode,
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
        let conn_ack = ConnAck::new(true, ReasonCode::Success);

        assert!(conn_ack.session_present());
        assert_eq!(ReasonCode::Success, conn_ack.reason_code());
    }
}
