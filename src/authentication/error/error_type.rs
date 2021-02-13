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
