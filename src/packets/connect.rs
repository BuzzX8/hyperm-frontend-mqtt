pub struct Connect {
    client_id: String,
    clean_start: bool,
    keep_alive: Option<u16>,
    user_name: Option<String>,
    password: Option<Vec<u8>>
}

impl Connect {
    pub fn new(client_id: &str) -> Connect {
        Connect {
            client_id: client_id.into(),
            clean_start: false,
            keep_alive: None,
            user_name: None,
            password: None
        }
    }

    pub fn client_id(&self) -> &str {        
        &self.client_id
    }

    pub fn clean_start(&self) -> bool {
        self.clean_start
    }

    pub fn set_clean_start(&mut self, clean_start: bool) {
        self.clean_start = clean_start;
    }

    pub fn keep_alive(&self) -> Option<u16> {
        self.keep_alive
    }

    pub fn set_keep_alive(&mut self, keep_alive: u16) {
        self.keep_alive = Some(keep_alive);
    }

    pub fn set_user_name(&mut self, user_name: &str) {
        self.user_name = Some(user_name.into());
    }

    pub fn set_password(&mut self, password: &[u8]) {
        self.password = Some(password.into());
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