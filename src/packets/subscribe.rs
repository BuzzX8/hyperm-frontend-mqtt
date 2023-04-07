use super::*;

pub struct Subscribe {
    id: PacketId,
}

impl Subscribe {
    pub fn new() -> Subscribe {
        Subscribe {
            id: new_packet_id(),
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
        let packet = Subscribe::new();

        assert_ne!(0, packet.id())
    }
}
