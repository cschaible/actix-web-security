//! The trait definition of a user details service and its clone capabilities for the JWT based authentication.

use async_trait::async_trait;
use downcast_rs::impl_downcast;
use downcast_rs::Downcast;

use crate::authentication::scheme::bearer::jwt::Claims;
use crate::user_details::UserDetails;

/// The trait definition of a user details service for the JWT based authentication.
/// A user details service is used to load the `UserDetails` for a given boxed `Claims`
/// object from a datastore. The `Claims` object contains all claims contained in the
/// decoded JWT.
#[async_trait]
pub trait JwtUserDetailsService: Downcast + JwtUserDetailsServiceClone {
    #[allow(clippy::borrowed_box)]
    async fn find_user(&self, token: &Box<dyn Claims>) -> Option<Box<dyn UserDetails>>;
}
impl_downcast!(JwtUserDetailsService);

pub trait JwtUserDetailsServiceClone: Send + Sync {
    fn clone_box(&self) -> Box<dyn JwtUserDetailsService>;
}

/// An user details service must be cloneable, `send` and `sync`.
/// Therefore it has to implement the `JwtUserDetailsServiceClone` trait to be cloneable as a boxed object.
impl<U> JwtUserDetailsServiceClone for U
where
    U: 'static + JwtUserDetailsService + Clone,
{
    fn clone_box(&self) -> Box<dyn JwtUserDetailsService> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn JwtUserDetailsService> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
