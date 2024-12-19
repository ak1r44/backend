use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    permissions::{Permission, PermissionSet},
    roles::Role,
};

/// The audience of the token.
const JWT_AUDIENCE: &str = "akira";
/// The duration of the token.
const TOKEN_DURATION: Duration = Duration::days(1);

#[derive(Debug, Clone)]
pub struct Token {
    pub(crate) raw: String,
    pub(crate) claims: Claims,
}

impl Token {
    pub fn uid(&self) -> Uuid {
        self.claims.uid
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub uid: Uuid,
    pub iat: i64,
    pub role: Role,
    pub permissions: PermissionSet,
    pub exp: i64,
}

impl Claims {
    pub fn new(uid: Uuid, role: Role) -> Self {
        Claims {
            uid,
            iat: Utc::now().timestamp(),
            role: role.clone(),
            permissions: role.permissions(),
            exp: Utc::now()
                .checked_add_signed(TOKEN_DURATION)
                .unwrap()
                .timestamp(),
        }
    }

    pub fn encode(&self, secret: &str) -> Result<Token, jsonwebtoken::errors::Error> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
        )?;

        Ok(Token {
            raw: token,
            claims: self.clone(),
        })
    }

    pub fn decode(token: String, secret: &str) -> Result<Token, jsonwebtoken::errors::Error> {
        let mut validation = jsonwebtoken::Validation::default();
        validation.set_audience(JWT_AUDIENCE.as_bytes());

        let decode = jsonwebtoken::decode::<Claims>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
            &validation,
        )?;

        Ok(Token {
            raw: token,
            claims: decode.claims,
        })
    }

    pub fn validate(&self) -> bool {
        Utc::now().timestamp() < self.exp
    }

    pub fn has(&self, permission: Permission) -> bool {
        self.permissions.contains(permission)
    }
}
