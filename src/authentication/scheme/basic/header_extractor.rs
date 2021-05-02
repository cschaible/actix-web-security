//! A default implementation of a authentication extractor for basic authentication.

use actix_web::http::{header, HeaderMap, HeaderValue};
use async_trait::async_trait;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::authentication::Authentication;
use crate::authentication::scheme::basic::BasicAuthentication;
use crate::authentication::scheme::header_extractor::{
    extract_auth_header, AuthorizationHeaderExtractor,
};

/// The definition of a `BasicAuthenticationExtractor`. The authentication extractor
/// extracts the authentication information from the authorization header and decodes
/// the user credentials to be used in the user authentication.
#[derive(Clone)]
pub struct BasicAuthenticationExtractor {}

impl BasicAuthenticationExtractor {
    /// Constructs a new instance of `BasicAuthenticationExtractor`.
    pub fn new() -> BasicAuthenticationExtractor {
        BasicAuthenticationExtractor {}
    }

    fn extract_basic(
        &self,
        header: &HeaderValue,
    ) -> Result<BasicAuthentication, AuthenticationError> {
        let token = extract_auth_header(header, "Basic", 7)?;

        // Decode the token
        if let Ok(decoded) = base64::decode(token) {
            let decoded_str = std::str::from_utf8(&decoded);
            if let Ok(credentials_str) = decoded_str {
                let mut credentials = credentials_str.splitn(2, ':');

                let username = credentials
                    .next()
                    .ok_or(AuthenticationError::InvalidAuthorizationHeader)?
                    .to_string();

                let password = credentials
                    .next()
                    .ok_or(AuthenticationError::InvalidAuthorizationHeader)?
                    .to_string();

                return Ok(BasicAuthentication { username, password });
            }
        }

        Err(AuthenticationError::InvalidAuthorizationHeader)
    }
}

impl Default for BasicAuthenticationExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl AuthorizationHeaderExtractor for BasicAuthenticationExtractor {
    async fn extract_token(
        &self,
        headers: &HeaderMap,
    ) -> Result<Box<dyn Authentication>, AuthenticationError> {
        let authorization_header = headers.get(header::AUTHORIZATION);
        let header_value =
            authorization_header.ok_or(AuthenticationError::AuthorizationHeaderNotSet)?;
        Ok(Box::new(self.extract_basic(header_value)?))
    }
}
