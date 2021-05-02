//! The jwt bearer scheme module offers an implementation of a JWT based authentication header extractor, authentication provider and
//! user detail service.

use crate::authentication::scheme::authentication::Authentication;
use crate::authentication::scheme::bearer::jwt::token::Claims;

pub mod authentication_provider;
pub mod default_jwt;
pub mod header_extractor;
pub mod token;
pub mod user_details_service;

/// A JWT authentication struct representing the decoded JWT `Claims` extracted from the authorization header.
pub struct JwtBearerAuthentication {
    pub token: Box<dyn Claims>,
}

impl Authentication for JwtBearerAuthentication {}
