use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;

#[derive(Debug, Clone)]
pub enum AuthContext {
    /// Internal tokens can generally be trusted.
    /// They are usually used by cockpit and expire fairly quickly.
    Internal { user_id: Id<UserMarker> },
    /// External tokens should not be trusted (e.g. perform permission checks)
    /// They are used by external services to interact with turbine
    External { user_id: Id<UserMarker> },
}

impl AuthContext {
    pub fn _user_id(&self) -> Id<UserMarker> {
        match self {
            Self::Internal { user_id } => *user_id,
            Self::External { user_id } => *user_id,
        }
    }

    pub fn is_internal(&self) -> bool {
        match self {
            Self::Internal { .. } => true,
            _ => false,
        }
    }
}

#[derive(Deserialize_repr)]
#[repr(u8)]
enum TokenType {
    Internal = 0,
    External = 1,
}

#[derive(Deserialize)]
struct TokenClaims {
    #[serde(rename = "tt")]
    pub kind: TokenType,
    #[serde(rename = "uid")]
    pub user_id: Id<UserMarker>,
    #[serde(rename = "exp")]
    pub _expiration: u64,
}

pub fn decode_auth_token(
    token: &str,
    secret: &str,
) -> Result<AuthContext, jsonwebtoken::errors::Error> {
    let data: TokenData<TokenClaims> = decode(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )?;

    let claims = data.claims;
    Ok(match claims.kind {
        TokenType::Internal => AuthContext::Internal {
            user_id: claims.user_id,
        },
        TokenType::External => AuthContext::External {
            user_id: claims.user_id,
        },
    })
}
