use std::future::Future;
use std::pin::Pin;

use actix_http::body::ResponseBody;
use actix_web::dev::PayloadStream;
use actix_web::dev::{MessageBody, Payload};
use actix_web::web::BytesMut;
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use actix_web_security::{
    authentication::error::error_type::AuthenticationError, user_details::UserDetails,
};

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
}

impl UserDetails for User {}

pub async fn deserialize_json<T, B>(body: &mut ResponseBody<B>) -> T
where
    B: MessageBody + Unpin,
    T: DeserializeOwned,
{
    let mut bytes = BytesMut::new();
    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }
    let body_bytes = bytes.freeze();

    serde_json::from_slice(&body_bytes)
        .unwrap_or_else(|_| panic!("read_response_json failed during deserialization"))
}

impl FromRequest for User {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut Payload<PayloadStream>) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            let extensions = req.extensions();
            let user_details = extensions.get::<Box<dyn UserDetails>>();

            let user = match user_details.cloned() {
                Some(boxed_user_details) => {
                    let cloned_user_details_box = boxed_user_details.clone_box();
                    cloned_user_details_box.downcast_ref::<User>().cloned()
                }
                None => None,
            };
            match user {
                Some(u) => Ok(u),
                None => Err(AuthenticationError::UsernameNotFound.into()),
            }
        })
    }
}
