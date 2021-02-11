use actix_web::http::{header, HeaderMap, HeaderValue};
use async_trait::async_trait;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::authentication::Authentication;
use crate::authentication::scheme::basic::BasicAuthentication;
use crate::authentication::scheme::header_extractor::{
    extract_auth_header, AuthorizationHeaderExtractor,
};

#[derive(Clone)]
pub struct BasicAuthenticationExtractor {}

impl BasicAuthenticationExtractor {
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
                    .ok_or(AuthenticationError::InvalidAuthorizationHeader)
                    .map(|username| username.to_string())?;

                let password = credentials
                    .next()
                    .ok_or(AuthenticationError::InvalidAuthorizationHeader)
                    .map(|password| password.to_string())?;

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
        match authorization_header {
            Some(header_value) => match self.extract_basic(header_value) {
                Ok(basic_auth) => Ok(Box::new(basic_auth)),
                Err(e) => Err(e),
            },
            None => Err(AuthenticationError::AuthorizationHeaderNotSet),
        }
    }
}
