use async_trait::async_trait;
use downcast_rs::impl_downcast;
use downcast_rs::Downcast;

use crate::authentication::scheme::bearer::jwt::Claims;
use crate::user_details::UserDetails;

#[async_trait]
pub trait JwtUserDetailsService: Downcast + JwtUserDetailsServiceClone {
    async fn find_user(&self, token: &Box<dyn Claims>) -> Option<Box<dyn UserDetails>>;
}
impl_downcast!(JwtUserDetailsService);

pub trait JwtUserDetailsServiceClone: Send + Sync {
    fn clone_box(&self) -> Box<dyn JwtUserDetailsService>;
}

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
