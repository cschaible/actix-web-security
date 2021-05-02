//! The decoder module provides a trait definition of `TokenDecoder` and a RSA token decoder implementation.

use serde::Deserialize;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::bearer::jwt::token::Claims;

pub mod rsa_decoder;

/// Token decoder claim trait definition. Decodes a string token to either a boxed instance of `Claims`
/// or returns an error.
pub trait TokenDecoder<T: for<'b> Deserialize<'b> + Claims>: TokenDecoderClone<T> {
    fn decode_token(&self, token: &str) -> Result<Box<T>, AuthenticationError>;
}

/// A token decoder must be cloneable, `send` and `sync`.
/// Therefore it has to implement the `TokenDecoderClone` trait to be cloneable as a boxed object.
pub trait TokenDecoderClone<T: for<'b> Deserialize<'b> + Claims>: Send + Sync {
    fn clone_box(&self) -> Box<dyn TokenDecoder<T>>;
}

impl<T: for<'b> Deserialize<'b> + Claims, U> TokenDecoderClone<T> for U
where
    U: 'static + TokenDecoder<T> + Clone,
{
    fn clone_box(&self) -> Box<dyn TokenDecoder<T>> {
        Box::new(self.clone())
    }
}

impl<T: for<'b> Deserialize<'b> + Claims> Clone for Box<dyn TokenDecoder<T>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
