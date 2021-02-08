use async_trait::async_trait;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::authentication::Authentication;
use crate::authentication::scheme::authentication_provider::AuthenticationProvider;
use crate::authentication::scheme::basic::BasicAuthentication;
use crate::authentication::scheme::basic::user_details_service::BasicUserDetailsService;
use crate::user_details::UserDetails;

#[derive(Clone)]
pub struct BasicAuthenticationProvider {
    user_details_service: Box<dyn BasicUserDetailsService>
}

impl BasicAuthenticationProvider {
    pub fn new(user_details_service: Box<dyn BasicUserDetailsService>) -> BasicAuthenticationProvider {
        BasicAuthenticationProvider {
            user_details_service
        }
    }
}

#[async_trait]
impl AuthenticationProvider for BasicAuthenticationProvider {
    #[allow(clippy::borrowed_box)]
    async fn authenticate(&self, authentication: &Box<dyn Authentication>) -> Result<Box<dyn UserDetails>, AuthenticationError> {
        if authentication.is::<BasicAuthentication>() {
            let basic_auth = authentication.downcast_ref::<BasicAuthentication>().unwrap();
            match self.user_details_service.find_user(&basic_auth.username, &basic_auth.password).await {
                Some(user) => Ok(user),
                None => Err(AuthenticationError::UsernameNotFound)
            }
        } else {
            Err(AuthenticationError::InvalidAuthentication)
        }
    }
}