//! The authentication module provides all authentication related functionality.
//! This consists of the actix middleware, authentication providers and default implementations
//! for OAuth2 and Basic authentication.

use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::authentication::Authentication;
use crate::authentication::scheme::authentication_provider::AuthenticationProvider;
use crate::user_details::UserDetails;

pub mod endpoint_matcher;
pub mod error;
pub mod middleware;
pub mod scheme;

/// A provider manager can be used to register one or more authentication providers to be executed in
/// a chain (until the authentication on a provider succeeds or fails on all providers).
/// A provider manager is registered in the middleware to execute the authentication process.
#[derive(Clone)]
pub struct ProviderManager {
    providers: Vec<Box<dyn AuthenticationProvider>>,
}

impl ProviderManager {
    /// Constructs a new instance for the given vector of boxed authentication providers.
    pub fn new(providers: Vec<Box<dyn AuthenticationProvider>>) -> ProviderManager {
        ProviderManager { providers }
    }

    #[allow(clippy::borrowed_box)]
    pub async fn authenticate(
        &self,
        authentication: &Box<dyn Authentication>,
    ) -> Result<Box<dyn UserDetails>, AuthenticationError> {
        let providers = &self.providers;
        let mut last_error: Option<AuthenticationError> = None;
        for provider in providers {
            let result = provider.authenticate(authentication).await;
            match result {
                Ok(user) => return Ok(user),
                Err(err) => {
                    last_error = Some(err);
                    continue;
                }
            }
        }
        match last_error {
            Some(e) => Err(e),
            None => Err(AuthenticationError::UsernameNotFound),
        }
    }
}
