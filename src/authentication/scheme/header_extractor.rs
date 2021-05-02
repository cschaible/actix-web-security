//! The authorization header trait definition and utility functions.

use actix_web::http::{HeaderMap, HeaderValue};
use async_trait::async_trait;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::authentication::Authentication;

/// The trait of `AuthorizationHeaderExtractor` to be implemented for a specific authentication scheme.
/// Takes a set of HTTP-Headers from the client request and extracts a token (in form of a boxed `Authentication`) from the headers.
#[async_trait]
pub trait AuthorizationHeaderExtractor: Send + Sync {
    async fn extract_token(
        &self,
        request: &HeaderMap,
    ) -> Result<Box<dyn Authentication>, AuthenticationError>;
}

/// Utility function to extract the actual token from the header for a given authentication scheme (basic/bearer).
/// Returns either a `String` with the extracted token (without the scheme prefix from the header) or an `AuthenticationError`.
pub fn extract_auth_header(
    header: &HeaderValue,
    auth_scheme: &str,
    header_length: usize,
) -> Result<String, AuthenticationError> {
    if header.len() < header_length {
        return Err(AuthenticationError::InvalidAuthorizationHeader);
    }

    // Split header into scheme (Basic/Bearer) and the actual token
    let token: &str;
    if let Ok(header_str) = header.to_str() {
        let mut parts = header_str.splitn(2, ' ');
        match parts.next() {
            Some(scheme) if scheme == auth_scheme => (),
            _ => return Err(AuthenticationError::InvalidAuthorizationHeader),
        }
        token = parts
            .next()
            .ok_or(AuthenticationError::InvalidAuthorizationHeader)?
    } else {
        return Err(AuthenticationError::InvalidAuthorizationHeader);
    }
    Ok(token.to_string())
}
