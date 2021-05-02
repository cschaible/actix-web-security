//! A default implementation of an `AuthenticationProvider` for a JWT based OAuth2 authentication.

use async_trait::async_trait;

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::authentication::Authentication;
use crate::authentication::scheme::authentication_provider::AuthenticationProvider;
use crate::authentication::scheme::bearer::jwt::user_details_service::JwtUserDetailsService;
use crate::authentication::scheme::bearer::jwt::JwtBearerAuthentication;
use crate::user_details::UserDetails;

/// The definition of a `JwtAuthenticationProvider`.
#[derive(Clone)]
pub struct JwtAuthenticationProvider {
    user_details_service: Box<dyn JwtUserDetailsService>,
}

impl JwtAuthenticationProvider {
    /// Constructs an instance of a `JwtAuthenticationProvider` for a boxed instance of a `JwtUserDetailsService`
    /// which does the actual data lookup for the authentication.
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
