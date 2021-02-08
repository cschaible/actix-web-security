use actix_web::http::{HeaderMap, HeaderValue};
use async_trait::async_trait;
use serde::Deserialize;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::authentication::Authentication;
use crate::authentication::scheme::bearer::jwt::{Claims, JwtBearerAuthentication};
use crate::authentication::scheme::bearer::jwt::token::decoder::TokenDecoder;
use crate::authentication::scheme::header_extractor::{AuthorizationHeaderExtractor, extract_auth_header};

#[derive(Clone)]
pub struct BearerAuthenticationExtractor<T: for<'b> Deserialize<'b> + Claims> {
    pub token_decoders: Vec<Box<dyn TokenDecoder<T>>>
}

impl<T: for<'b> Deserialize<'b> + Claims> BearerAuthenticationExtractor<T> {
    pub fn new(token_decoders: Vec<Box<dyn TokenDecoder<T>>>) -> BearerAuthenticationExtractor<T> {
        BearerAuthenticationExtractor {
            token_decoders
        }
    }

    fn extract_bearer(&self, header: &HeaderValue) -> Result<String, AuthenticationError> {
        extract_auth_header(header, "Bearer", 8)
    }
}

#[async_trait]
impl<T: for<'b> Deserialize<'b> + Claims> AuthorizationHeaderExtractor for BearerAuthenticationExtractor<T> {
    async fn extract_token(&self, headers: &HeaderMap) -> Result<Box<dyn Authentication>, AuthenticationError> {
        let authorization_header = headers.get("authorization");
        match authorization_header {
            Some(header_value) => {
                match self.extract_bearer(header_value) {
                    Ok(extracted_token) => {
                        for decoder in &self.token_decoders {
                            if let Ok(decoded_token) = decoder.decode_token(&extracted_token) {
                                return Ok(Box::new(JwtBearerAuthentication {
                                    token: decoded_token
                                }));
                            }
                        }
                        Err(AuthenticationError::InvalidAuthentication)
                    }
                    Err(error) => Err(error)
                }
            }
            None => Err(AuthenticationError::AuthorizationHeaderNotSet)
        }
    }
}