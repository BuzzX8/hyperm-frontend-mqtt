use super::*;

pub struct Unsubscribe {
    id: PacketId,
    topic_filters: Vec<String>,
}

impl Unsubscribe {
    pub fn new(topic_filters: &[&str]) -> Unsubscribe {
        Unsubscribe {
            id: new_packet_id(),
            topic_filters: topic_filters.iter().map(|f| (*f).into()).collect(),
        }
    }

    pub fn id(&self) -> PacketId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::Unsubscribe;

    #[test]
    fn new_unsubscribe_creates_packet() {
        let topic_filters = ["topic-1", "topic-2", "topic-3"];
        let unsubscribe = Unsubscribe::new(&topic_filters);

        assert_ne!(0, unsubscribe.id());
    }
}
