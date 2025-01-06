use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StreamUser {
    username: String,
    password: String,
}

impl StreamUser {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl StreamUser {
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

impl Default for StreamUser {
    fn default() -> Self {
        Self::new("default_user", "default_password")
    }
}

impl fmt::Display for StreamUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User {{ username: {} }}", self.username,)
    }
}
