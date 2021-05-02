//! The token module provides a trait definition of `Claims`, `TokenDecoder` and a RSA token decoder implementation.

use downcast_rs::impl_downcast;
use downcast_rs::Downcast;

pub mod decoder;

/// Trait definition of claims to decode from a token.
pub trait Claims: Downcast + Sync + Send {}
impl_downcast!(Claims);
