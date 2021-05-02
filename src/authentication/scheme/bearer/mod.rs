//! The bearer scheme module offers an implementation of JWT based OAuth2 authentication header extractor,
//! authentication provider and user detail service.  
//! Additionally a JWK module is provided to load JWKs for token verification from the local file system
//! or dynamically from a HTTP-Endpoint.

pub mod jwk;
pub mod jwt;
