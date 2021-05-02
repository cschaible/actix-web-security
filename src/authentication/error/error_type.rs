//! Enumerations of error types.

use derive_more::{Display, Error};

/// Authentication related errors.
#[derive(Debug, Display, Error, PartialEq, Eq, Hash, Clone)]
pub enum AuthenticationError {
    /// No `Authorization` header is set.
    #[display(fmt = "Authorization header not set")]
    AuthorizationHeaderNotSet,

    /// The value in the `Authorization` header is invalid.
    #[display(fmt = "Invalid Authorization header")]
    InvalidAuthorizationHeader,

    /// The type of authorization (Basic/Bearer/etc.) cannot be handled by the current authentication provider or header extractor.
    #[display(fmt = "Access denied")]
    InvalidAuthentication,

    /// The authentication data/token cannot be decoded successfully or is not valid anymore.
    #[display(fmt = "Invalid authentication type")]
    InvalidToken,

    /// A user to be authenticated cannot be found or cannot be retrieved from the request context/extensions.
    #[display(fmt = "Access denied")]
    UsernameNotFound,
}

/// Errors related to JWT loading problems.
#[derive(Debug, Display, Error, PartialEq, Eq, Hash, Clone)]
pub enum JwkLoaderError {
    /// The keyfile cannot be read from the local file system.
    #[display(fmt = "The key file could not be read")]
    KeyFileCouldNotBeRead,

    /// The keyfile is invalid.
    #[display(fmt = "Invalid key file")]
    InvalidKeyFile,

    /// The downloaded JWKs are invalid.
    #[display(fmt = "Invalid JSON response")]
    InvalidJsonResponse,

    /// The JWKs couldn't be downloaded.
    #[display(fmt = "JWKs could not be downloaded")]
    JwksCouldNotBeDownloaded,
}
