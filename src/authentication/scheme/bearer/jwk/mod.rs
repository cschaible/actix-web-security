use std::fs;

use serde::Deserialize;

pub mod default_jwk;

#[cfg(feature = "jwk-default-loader")]
pub mod default_jwk_loader;

pub struct JwkLoader<T: for<'a> Deserialize<'a>> {
    pub jwks: T,
}

impl<T: for<'a> Deserialize<'a>> JwkLoader<T> {
    // i don't like that those functions can panic
    pub fn from_file(filename: String) -> JwkLoader<T> {
        let key = fs::read_to_string(filename).expect("Key file couldn't be read");
        let jwks: T = serde_json::from_str(key.as_str()).unwrap();
        JwkLoader { jwks }
    }

    #[cfg(feature = "jwk-loader")]
    pub fn from_url(url: String) -> JwkLoader<T> {
        let jwks: T = reqwest::blocking::get(&url).unwrap().json::<T>().unwrap();
        JwkLoader { jwks }
    }
}
