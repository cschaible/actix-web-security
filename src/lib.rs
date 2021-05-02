//! # actix-web-security
//! Basic-Auth / OAuth2 easy-to-use authentication modules for actix web.
//!
//! ## Features
//!
//! * HTTP Authentication with the following authentication schemes
//!   * Basic Authentication
//!   * Bearer Authentication
//! * OAuth2 Resource Server "Auto-Configuration"
//! * JWK-Downloader to verify JWTs
//! * JWT verification
//!
//! ## Note: Neither audited nor penetration tested
//! This library is provided "as is" without warranties of any kind and is not verified to be secure.
//! It has neither been audited to be safe in an audit nor been penetration tested.
//! The library was developed to the best of knowledge and belief.
//! It's in your own responsibility to check the code for potential security issues or bugs and your own decision
//! whether you see the code as safe and trustworthy or whether you prefer to not use it.
//! The library is provided as open-source and the liability of any kind is excluded as described in the licenses
//! the software is provided under.
//!
//! ## Install
//! Add the following dependency to your `cargo.toml`.
//!
//! ```toml
//! actix-web-security = "0.1.0"
//! ```
//!
//! The following features can be activated:
//! * `jwk-loader`  
//!   This feature can be activated to download custom JWKs from an authorization server
//!   ```toml
//!   actix-web-security = { version="0.1.0", features = ["jwk-loader"] }
//!   ```
//!
//! * `jwk-default-loader`  
//!   This feature can be activated to download `DefaultJwks` from an authorization server.
//!   ```toml
//!   actix-web-security = { version="0.1.0", features = ["jwk-default-loader"] }
//!   ```
//!
//! Both features require `openssl` to be installed on the system.
//! The documentation about how to install it can be found [here](https://docs.rs/openssl/0.10.32/openssl/#automatic).
//!
//! ## Samples
//! Sample applications can be found [here](https://github.com/cschaible/actix-web-security-samples).

pub mod authentication;
pub mod user_details;
