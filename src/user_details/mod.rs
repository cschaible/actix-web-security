//! The user_details module provides all user retrieval related functionality that is required for the authentication process.

use downcast_rs::impl_downcast;
use downcast_rs::Downcast;

pub mod attachment;
pub mod request_extension;

/// Marker trait for a user object to put into the request context.
pub trait UserDetails: Downcast + UserDetailsClone {}
impl_downcast!(UserDetails);

/// A user details object must be cloneable.
/// Therefore it has to implement the `UserDetailsClone` trait to be cloneable as a boxed object.
pub trait UserDetailsClone {
    fn clone_box(&self) -> Box<dyn UserDetails>;
}

impl<U> UserDetailsClone for U
where
    U: 'static + UserDetails + Clone,
{
    fn clone_box(&self) -> Box<dyn UserDetails> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn UserDetails> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
