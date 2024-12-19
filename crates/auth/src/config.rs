use std::env;

pub struct Config {
    pub jwt_secret: String,
    pub pepper: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let pepper = env::var("PWD_PEPPER").expect("PWD_PEPPER must be set");

        Self { jwt_secret, pepper }
    }
}
