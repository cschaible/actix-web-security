//! The basic scheme module offers an implementation of a basic authentication header extractor, authentication provider and
//! user detail service.

use crate::authentication::scheme::authentication::Authentication;

pub mod authentication_provider;
pub mod header_extractor;
pub mod user_details_service;

/// A basic authentication struct representing the username and password extracted from the authorization header.
pub struct BasicAuthentication {
    pub username: String,
    pub password: String,
}

impl Authentication for BasicAuthentication {}
