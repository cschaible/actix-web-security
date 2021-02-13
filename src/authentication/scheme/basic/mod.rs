use crate::authentication::scheme::authentication::Authentication;

pub mod authentication_provider;
pub mod header_extractor;
pub mod user_details_service;

pub struct BasicAuthentication {
    pub username: String,
    pub password: String,
}

impl Authentication for BasicAuthentication {}
