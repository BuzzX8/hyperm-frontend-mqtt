use super::*;

pub struct Subscribe {
    id: PacketId,
    requests: Vec<Request>,
}

#[derive(Clone)]
pub struct Request {
    topic_filter: String,
    max_qos: Qos,
    no_local: bool,
    retain_as_published: bool,
    retain_handling_option: RetainHandlingOption
}

#[derive(Clone)]
pub enum RetainHandlingOption {
    SendAtSubscribeTime,
    SendIfSubscriptionDoesNotExists,
    DonNotSend
}

impl Subscribe {
    pub fn new(requests: &[Request]) -> Subscribe {
        Subscribe {
            id: new_packet_id(),
            requests: requests.into(),
        }
    }

    pub fn id(&self) -> PacketId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::Subscribe;

    #[test]
    fn new_creates_subscribe() {
        let packet = Subscribe::new(&[]);

        assert_ne!(0, packet.id())
    }
}
