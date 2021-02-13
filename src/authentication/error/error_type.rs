use derive_more::{Display, Error};

#[derive(Debug, Display, Error, PartialEq, Eq, Hash, Clone)]
pub enum AuthenticationError {
    #[display(fmt = "Authorization header not set")]
    AuthorizationHeaderNotSet,
    #[display(fmt = "Invalid Authorization header")]
    InvalidAuthorizationHeader,
    #[display(fmt = "Access denied")]
    InvalidAuthentication,
    #[display(fmt = "Invalid authentication type")]
    InvalidToken,
    #[display(fmt = "Access denied")]
    UsernameNotFound,
}

#[derive(Debug, Display, Error, PartialEq, Eq, Hash, Clone)]
pub enum JwkLoaderError {
    #[display(fmt = "The key file could not be read")]
    KeyFileCouldNotBeRead,
    #[display(fmt = "Invalid key file")]
    InvalidKeyFile,
    #[display(fmt = "Invalid JSON response")]
    InvalidJsonResponse,
    #[display(fmt = "JWKs could not be downloaded")]
    JwksCouldNotBeDownloaded,
}
