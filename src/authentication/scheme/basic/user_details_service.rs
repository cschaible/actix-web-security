use async_trait::async_trait;
use downcast_rs::impl_downcast;
use downcast_rs::Downcast;

use crate::user_details::UserDetails;

#[async_trait]
pub trait BasicUserDetailsService: Downcast + BasicUserDetailsServiceClone {
    async fn find_user(&self, username: &str, password: &str) -> Option<Box<dyn UserDetails>>;
}
impl_downcast!(BasicUserDetailsService);

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
