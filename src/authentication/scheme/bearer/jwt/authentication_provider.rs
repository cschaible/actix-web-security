use async_trait::async_trait;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::authentication::Authentication;
use crate::authentication::scheme::authentication_provider::AuthenticationProvider;
use crate::authentication::scheme::bearer::jwt::user_details_service::JwtUserDetailsService;
use crate::authentication::scheme::bearer::jwt::JwtBearerAuthentication;
use crate::user_details::UserDetails;

#[derive(Clone)]
pub struct JwtAuthenticationProvider {
    user_details_service: Box<dyn JwtUserDetailsService>,
}

impl JwtAuthenticationProvider {
    pub fn new(user_details_service: Box<dyn JwtUserDetailsService>) -> JwtAuthenticationProvider {
        JwtAuthenticationProvider {
            user_details_service,
        }
    }
}

#[async_trait]
impl AuthenticationProvider for JwtAuthenticationProvider {
    #[allow(clippy::borrowed_box)]
    async fn authenticate(
        &self,
        authentication: &Box<dyn Authentication>,
    ) -> Result<Box<dyn UserDetails>, AuthenticationError> {
        if authentication.is::<JwtBearerAuthentication>() {
            let jwt_auth = authentication
                .downcast_ref::<JwtBearerAuthentication>()
                .unwrap();
            match self.user_details_service.find_user(&jwt_auth.token).await {
                Some(user) => Ok(user),
                None => Err(AuthenticationError::UsernameNotFound),
            }
        } else {
            Err(AuthenticationError::InvalidAuthentication)
        }
    }
}
