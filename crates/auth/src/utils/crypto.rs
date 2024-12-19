use aes_gcm::aead::{rand_core::RngCore, OsRng};
use argon2::Config;
use thiserror::Error;

/// The length of the salt (32 bytes = 256 bits).
const SALT_LENGTH: usize = 32;
/// The maximum length of a password.
const MAX_PASSWORD_LENGTH: usize = 128;
/// The minimum length of a password.
const MIN_PASSWORD_LENGTH: usize = 8;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("Error hashing the password: {0}")]
    HashError(String),

    #[error("Error verifying the password: {0}")]
    VerifyError(String),

    #[error(
        "Invalid password length ({}-{} characters)",
        MIN_PASSWORD_LENGTH,
        MAX_PASSWORD_LENGTH
    )]
    PasswordLengthError,
}

pub struct CryptoUtil {
    /// @see [Pepper (cryptography)](https://en.wikipedia.org/wiki/Pepper_(cryptography))
    pub pepper: String,
}

impl CryptoUtil {
    pub fn new() -> Result<Self, CryptoError> {
        let pepper = "".to_string();
        Ok(Self { pepper })
    }

    /// Hash the password.
    pub fn hash(&self, password: &str) -> Result<String, CryptoError> {
        if password.len() < MIN_PASSWORD_LENGTH || password.len() > MAX_PASSWORD_LENGTH {
            return Err(CryptoError::PasswordLengthError);
        }

        let salt: Vec<u8> = (0..SALT_LENGTH).map(|_| OsRng.next_u32() as u8).collect();
        let password_with_pepper = format!("{}{}", password, self.pepper);

        argon2::hash_encoded(password_with_pepper.as_bytes(), &salt, &Config::default())
            .map_err(|e| CryptoError::HashError(e.to_string()))
    }

    /// Verify the password against the hash.
    pub fn verify(&self, password: &str, hash: &str) -> Result<bool, CryptoError> {
        let password_with_pepper = format!("{}{}", password, self.pepper);

        argon2::verify_encoded(hash, password_with_pepper.as_bytes())
            .map_err(|e| CryptoError::VerifyError(e.to_string()))
    }
}
