use serde::Deserialize;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::bearer::jwt::token::Claims;

pub mod rsa_decoder;

pub trait TokenDecoder<T: for<'b> Deserialize<'b> + Claims>: TokenDecoderClone<T> {
    fn decode_token(&self, token: &str) -> Result<Box<T>, AuthenticationError>;
}

pub trait TokenDecoderClone<T: for<'b> Deserialize<'b> + Claims>: Send + Sync {
    fn clone_box(&self) -> Box<dyn TokenDecoder<T>>;
}

impl<T: for<'b> Deserialize<'b> + Claims, U> TokenDecoderClone<T> for U
    where U: 'static + TokenDecoder<T> + Clone {
    fn clone_box(&self) -> Box<dyn TokenDecoder<T>> {
        Box::new(self.clone())
    }
}

impl<T: for<'b> Deserialize<'b> + Claims> Clone for Box<dyn TokenDecoder<T>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}