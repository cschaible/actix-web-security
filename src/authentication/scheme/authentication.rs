//! The trait definition of authentications.
use downcast_rs::impl_downcast;
use downcast_rs::Downcast;

/// The marker trait of an `Authentication` implementation. A `Authentication` object is a DTO that is used to transfer the extracted
/// credentials / token data from the header to the actual authentication.
pub trait Authentication: Downcast + Sync + Send {}
impl_downcast!(Authentication);
