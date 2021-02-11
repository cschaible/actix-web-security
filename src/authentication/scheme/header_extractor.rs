use actix_web::http::{HeaderMap, HeaderValue};
use async_trait::async_trait;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::authentication::Authentication;

#[async_trait]
pub trait AuthorizationHeaderExtractor: Send + Sync {
    async fn extract_token(
        &self,
        request: &HeaderMap,
    ) -> Result<Box<dyn Authentication>, AuthenticationError>;
}

pub fn extract_auth_header<'a>(
    header: &'a HeaderValue,
    auth_scheme: &str,
    header_length: usize,
) -> Result<&'a str, AuthenticationError> {
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
    Ok(token)
}
