pub struct Connect {
    client_id: String,
    clean_start: bool,
    keep_alive: Option<u16>,
    user_name: Option<String>,
    //password: Option<&[u8]>
}

impl Connect {
    fn new(client_id: &str) -> Connect {
        Connect {
            client_id: client_id.to_owned(),
            clean_start: false,
            keep_alive: Option::None,
            user_name: Option::None
        }
    }

    fn client_id(&self) -> &str {        
        &self.client_id
    }

    fn clean_start(&self) -> bool {
        self.clean_start
    }

    fn set_clean_start(&mut self, clean_start: bool) {
        self.clean_start = clean_start;
    }

    fn keep_alive(&self) -> Option<u16> {
        self.keep_alive
    }

    fn set_keep_alive(&mut self, keep_alive: u16) {
        self.keep_alive = Some(keep_alive);
    }
}

#[cfg(test)]
mod tests {
    use super::Connect;

    #[test]
    fn new_creates_connect_with_client_id() {
        let client_id = "some-client-id";
        let connect = Connect::new(client_id);

        assert_eq!(client_id, connect.client_id());
    }
}