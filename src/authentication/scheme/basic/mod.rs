use crate::authentication::scheme::authentication::Authentication;

pub mod authentication_provider;
pub mod user_details_service;
pub mod header_extractor;

pub struct BasicAuthentication {
    pub username: String,
    pub password: String,
}

impl Authentication for BasicAuthentication {}