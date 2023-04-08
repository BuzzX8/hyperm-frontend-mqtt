use super::*;

pub struct Subscribe {
    id: PacketId,
    requests: Vec<Request>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Request {
    topic_filter: String,
    max_qos: Qos,
    no_local: bool,
    retain_as_published: bool,
    retain_handling_option: RetainHandlingOption,
}

#[derive(Clone, Debug, PartialEq)]
pub enum RetainHandlingOption {
    SendAtSubscribeTime,
    SendIfSubscriptionDoesNotExists,
    DonNotSend,
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

    pub fn requests(&self) -> &[Request] {
        &self.requests
    }
}

impl Request {
    pub fn new(topic_filter: &str, max_qos: Qos) -> Request {
        Request {
            topic_filter: topic_filter.into(),
            max_qos,
            no_local: false,
            retain_as_published: false,
            retain_handling_option: RetainHandlingOption::SendAtSubscribeTime,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{subscribe::Request, Qos};

    use super::Subscribe;

    #[test]
    fn new_creates_subscribe() {
        let requests = [
            Request::new("topic-0", Qos::Qos0),
            Request::new("topic-1", Qos::Qos1),
            Request::new("topic-2", Qos::Qos2),
        ];
        let packet = Subscribe::new(&requests);

        assert_ne!(0, packet.id());
        assert_eq!(&requests[..], packet.requests());
    }
}
