use async_trait::async_trait;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::authentication::Authentication;
use crate::user_details::UserDetails;

#[async_trait]
pub trait AuthenticationProvider: AuthenticationProviderClone {
    #[allow(clippy::borrowed_box)]
    async fn authenticate(
        &self,
        authentication: &Box<dyn Authentication>,
    ) -> Result<Box<dyn UserDetails>, AuthenticationError>;
}

pub trait AuthenticationProviderClone: Send + Sync {
    fn clone_box(&self) -> Box<dyn AuthenticationProvider>;
}

impl<U> AuthenticationProviderClone for U
where
    U: 'static + AuthenticationProvider + Clone,
{
    fn clone_box(&self) -> Box<dyn AuthenticationProvider> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AuthenticationProvider> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
