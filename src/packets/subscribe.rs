use super::{subscription_request::SubscriptionRequest, *};

pub struct Subscribe {
    id: PacketId,
    requests: Vec<SubscriptionRequest>,
}

impl Subscribe {
    pub fn new(requests: &[SubscriptionRequest]) -> Subscribe {
        let mut reqs: Vec<SubscriptionRequest> = Vec::with_capacity(requests.len());
        reqs.extend_from_slice(requests);

        Subscribe {
            id: new_packet_id(),
            requests: reqs,
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
