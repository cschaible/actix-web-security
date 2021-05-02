//! A JWT is characterized through a set of `Claims`. There are mandatory claims defined for JWTs and optional ones.
//! This module provides a default implementation with common claims (iss, sub, aud, exp, nbf, iat, jti).
use serde::{Deserialize, Serialize};

use crate::authentication::scheme::bearer::jwt::token::Claims;

/// A default implementation that can be used for JWT based authentication with commonly used claims.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefaultJwt {
    /// The URL of the identity provider
    pub iss: Option<String>,
    /// The principal identifier
    pub sub: Option<String>,
    /// The recipients the claim is for
    pub aud: Option<String>,
    /// The expiration date of the token
    pub exp: Option<usize>,
    /// The time the token must not be used before
    pub nbf: Option<usize>,
    /// The time the token was issues
    pub iat: Option<usize>,
    /// Unique identifier of the token
    pub jti: Option<String>,
}

impl Claims for DefaultJwt {}
