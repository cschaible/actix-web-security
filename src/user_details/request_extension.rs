//! Extension function to inject a `UserDetails` object into an endpoint.

use std::future::Future;
use std::pin::Pin;

use actix_web::dev::{Payload, PayloadStream};
use actix_web::{Error, FromRequest, HttpRequest};

use crate::authentication::error::error_type::AuthenticationError;
use crate::user_details::UserDetails;

impl FromRequest for Box<dyn UserDetails> {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload<PayloadStream>) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            req.extensions()
                .get::<Box<dyn UserDetails>>()
                .map(|x| x.clone_box())
                .ok_or_else(|| AuthenticationError::UsernameNotFound.into())
        })
    }
}
