extern crate base64;

pub struct Session {
    pub api_key: String,
}

impl Session {
    pub fn new(api_key: &'static str) -> Self {
        Session {
            api_key: api_key.to_string(),
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.api_key.is_empty()
    }

    pub fn auth_string(&self) -> String {
        let auth = format!("{}:api_token", self.api_key);
        base64::encode(auth)
    }
}

#[cfg(test)]
mod test {
    use super::Session;

    #[test]
    pub fn session_is_valid() {
        let mut session = Session::new("abc123");
        assert!(session.is_valid());

        session.api_key = String::new();
        assert!(!session.is_valid());
    }
}
