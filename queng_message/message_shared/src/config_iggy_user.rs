use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct IggyUser {
    username: String,
    password: String,
}

impl IggyUser {
    /// Creates a new `IggyUser` with the given `username` and `password`
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user
    /// * `password` - The password of the user
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

impl IggyUser {
    /// Returns the username of the user
    pub fn username(&self) -> &str {
        &self.username
    }
    /// Returns the password of the user
    pub fn password(&self) -> &str {
        &self.password
    }
}

impl Default for IggyUser {
    fn default() -> Self {
        Self::new("iggy", "iggy")
    }
}

impl fmt::Display for IggyUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User {{ username: {} }}", self.username,)
    }
}
