use crate::authentication::scheme::authentication::Authentication;
use crate::authentication::scheme::bearer::jwt::token::Claims;

pub mod authentication_provider;
pub mod default_jwt;
pub mod user_details_service;
pub mod header_extractor;
pub mod token;

pub struct JwtBearerAuthentication {
    pub token: Box<dyn Claims>
}

impl Authentication for JwtBearerAuthentication {}