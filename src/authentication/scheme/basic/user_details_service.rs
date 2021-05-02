//! The trait definition of a user details service and its clone capabilities for basic authentication.

use async_trait::async_trait;
use downcast_rs::impl_downcast;
use downcast_rs::Downcast;

use crate::user_details::UserDetails;

/// The trait definition of a user details service for basic authentication.
/// A user details service is used to load the `UserDetails` for a given username
/// and passwort from a datastore.
#[async_trait]
pub trait BasicUserDetailsService: Downcast + BasicUserDetailsServiceClone {
    async fn find_user(&self, username: &str, password: &str) -> Option<Box<dyn UserDetails>>;
}
impl_downcast!(BasicUserDetailsService);

/// An user details service must be cloneable, `send` and `sync`.
/// Therefore it has to implement the `BasicUserDetailsServiceClone` trait to be cloneable as a boxed object.
pub trait BasicUserDetailsServiceClone: Sync + Send {
    fn clone_box(&self) -> Box<dyn BasicUserDetailsService>;
}

impl<U> BasicUserDetailsServiceClone for U
where
    U: 'static + BasicUserDetailsService + Clone,
{
    fn clone_box(&self) -> Box<dyn BasicUserDetailsService> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn BasicUserDetailsService> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
