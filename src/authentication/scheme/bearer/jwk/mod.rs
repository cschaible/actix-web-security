use std::fs;

use serde::Deserialize;

use crate::authentication::error::error_type::JwkLoaderError;

pub mod default_jwk;

#[cfg(feature = "jwk-default-loader")]
pub mod default_jwk_loader;

pub struct JwkLoader<T: for<'a> Deserialize<'a>> {
    pub jwks: T,
}

impl<T: for<'a> Deserialize<'a>> JwkLoader<T> {
    pub fn from_file(filename: String) -> Result<JwkLoader<T>, JwkLoaderError> {
        match fs::read_to_string(filename) {
            Ok(key) => match serde_json::from_str(key.as_str()) {
                Ok(jwks) => Ok(JwkLoader { jwks }),
                Err(_) => Err(JwkLoaderError::InvalidKeyFile),
            },
            Err(_) => Err(JwkLoaderError::KeyFileCouldNotBeRead),
        }
    }

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
