use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use serde::Deserialize;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::bearer::jwt::token::Claims;
use crate::authentication::scheme::bearer::jwt::token::decoder::TokenDecoder;

pub trait RsaKeyComponents {
    fn get_n(&self) -> String;
    fn get_e(&self) -> String;
}

#[derive(Clone)]
pub struct RsaJwtDecoder {
    algorithm: Algorithm,
    decoding_keys: Vec<DecodingKey<'static>>,
}

impl RsaJwtDecoder {
    pub fn new(algorithm: Algorithm, rsa_keys: Vec<Box<dyn RsaKeyComponents>>) -> RsaJwtDecoder {
        let mut decoding_keys: Vec<DecodingKey<'static>> = Vec::new();

        for rsa_key in &rsa_keys {
            let n: String = rsa_key.get_n();
            let e: String = rsa_key.get_e();
            // It is important to call into_static(). Otherwise there are problems with the lifetimes of n and e.
            let decoding_key = DecodingKey::from_rsa_components(n.as_ref(), e.as_ref()).into_static();
            decoding_keys.push(decoding_key);
        }

        RsaJwtDecoder {
            algorithm,
            decoding_keys,
        }
    }
}

impl<T: for<'b> Deserialize<'b> + Claims> TokenDecoder<T> for RsaJwtDecoder {
    fn decode_token(&self, token: &str) -> Result<Box<T>, AuthenticationError> {
        for key in &self.decoding_keys {
            let result = decode::<T>(token, key, &Validation::new(self.algorithm));
            if let Ok(decoded_token) = result {
                return Ok(Box::new(decoded_token.claims));
            }
        }
        Err(AuthenticationError::InvalidToken)
    }
}