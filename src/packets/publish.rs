use super::{new_packet_id, PacketId, Qos};

pub struct Publish {
    id: PacketId,
    dup: bool,
    qos: Qos,
    retain: bool,
    topic_name: String,
    payload: Vec<u8>,
}

impl Publish {
    pub fn new(topic_name: &str, qos: Qos, payload: &[u8]) -> Publish {
        Publish {
            id: new_packet_id(),
            dup: false,
            qos,
            retain: false,
            topic_name: topic_name.into(),
            payload: payload.into(),
        }
    }

    pub fn id(&self) -> PacketId {
        self.id
    }

    pub fn qos(&self) -> Qos {
        self.qos
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_publish_packet() {
        let topic_name = "publish-topic";
        let qos = Qos::Qos1;
        let payload = [1u8, 2, 3, 4, 5];

        let publish = Publish::new(topic_name, qos, &payload);

        assert_ne!(0, publish.id());
        assert_eq!(qos, publish.qos());
        assert_eq!(payload, publish.payload());
    }
}