use actix_web::http::{HeaderMap, HeaderValue};
use async_trait::async_trait;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::authentication::Authentication;
use crate::authentication::scheme::basic::BasicAuthentication;
use crate::authentication::scheme::header_extractor::{AuthorizationHeaderExtractor, extract_auth_header};

#[derive(Clone)]
pub struct BasicAuthenticationExtractor {}

impl BasicAuthenticationExtractor {
    pub fn new() -> BasicAuthenticationExtractor {
        BasicAuthenticationExtractor {}
    }

    fn extract_basic(&self, header: &HeaderValue) -> Result<(String, String), AuthenticationError> {
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

                return Ok((username, password));
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
    async fn extract_token(&self, headers: &HeaderMap) -> Result<Box<dyn Authentication>, AuthenticationError> {
        let authorization_header = headers.get("authorization");
        match authorization_header {
            Some(header_value) => {
                match self.extract_basic(header_value) {
                    Ok(extracted_token) => Ok(Box::new(BasicAuthentication {
                        username: extracted_token.0,
                        password: extracted_token.1,
                    })),
                    Err(e) => Err(e)
                }
            }
            None => Err(AuthenticationError::AuthorizationHeaderNotSet)
        }
    }
}