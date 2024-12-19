use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    UserNotFound,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

pub struct AuthService {}

impl AuthService {
    pub fn new() -> Self {
        AuthService {}
    }

    pub fn login(&self, username: &str, password: &str) -> Option<AuthResponse> {
        todo!()
    }
}
