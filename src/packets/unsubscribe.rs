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
