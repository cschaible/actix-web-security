//! The jwk module provides utility functions to load JWKs to use for verification of JWTs.

use std::fs;

use serde::Deserialize;

use crate::authentication::error::error_type::JwkLoaderError;

pub mod default_jwk;

#[cfg(feature = "jwk-default-loader")]
pub mod default_jwk_loader;

/// JWK loader definition
pub struct JwkLoader<T: for<'a> Deserialize<'a>> {
    pub jwks: T,
}

impl<T: for<'a> Deserialize<'a>> JwkLoader<T> {
    /// Load a JWK file from disk.
    pub fn from_file(filename: String) -> Result<JwkLoader<T>, JwkLoaderError> {
        match fs::read_to_string(filename) {
            Ok(key) => match serde_json::from_str(key.as_str()) {
                Ok(jwks) => Ok(JwkLoader { jwks }),
                Err(_) => Err(JwkLoaderError::InvalidKeyFile),
            },
            Err(_) => Err(JwkLoaderError::KeyFileCouldNotBeRead),
        }
    }

    /// Download a JWK file from a remote location with http.
    #[cfg(feature = "jwk-loader")]
    pub fn from_url(url: String) -> Result<JwkLoader<T>, JwkLoaderError> {
        match reqwest::blocking::get(&url) {
            Ok(response) => match response.json::<T>() {
                Ok(jwks) => Ok(JwkLoader { jwks }),
                Err(_) => Err(JwkLoaderError::InvalidJsonResponse),
            },
            Err(_) => Err(JwkLoaderError::JwksCouldNotBeDownloaded),
        }
    }
}
