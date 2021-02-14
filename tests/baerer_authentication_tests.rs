use std::future::Future;
use std::pin::Pin;

use actix_http::body::ResponseBody;
use actix_web::dev::PayloadStream;
use actix_web::dev::Service;
use actix_web::dev::{MessageBody, Payload};
use actix_web::http::header;
use actix_web::web::BytesMut;
use actix_web::{get, test, App, HttpResponse, Responder};
use actix_web::{Error, FromRequest, HttpRequest};
use async_trait::async_trait;
use futures_util::StreamExt;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use actix_web_security::authentication::endpoint_matcher::AllEndpointsMatcher;
use actix_web_security::authentication::error::error_type::AuthenticationError;
use actix_web_security::authentication::middleware::HttpAuthenticationMiddleware;
use actix_web_security::authentication::scheme::bearer::jwt::authentication_provider::JwtAuthenticationProvider;
use actix_web_security::authentication::scheme::bearer::jwt::default_jwt::DefaultJwt;
use actix_web_security::authentication::scheme::bearer::jwt::header_extractor::BearerAuthenticationExtractor;
use actix_web_security::authentication::scheme::bearer::jwt::token::decoder::TokenDecoder;
use actix_web_security::authentication::scheme::bearer::jwt::token::Claims;
use actix_web_security::authentication::scheme::bearer::jwt::user_details_service::JwtUserDetailsService;
use actix_web_security::authentication::ProviderManager;
use actix_web_security::user_details::UserDetails;

#[derive(Clone, Eq, PartialEq, Debug, Deserialize, Serialize)]
struct User {
    username: String,
}

impl UserDetails for User {}

#[derive(Clone)]
struct JwtUserDetailsServiceImpl {}

#[async_trait]
impl JwtUserDetailsService for JwtUserDetailsServiceImpl {
    async fn find_user(&self, token: &Box<dyn Claims>) -> Option<Box<dyn UserDetails>> {
        if token.is::<DefaultJwt>() {
            if let Some(claims) = token.downcast_ref::<DefaultJwt>() {
                if let Some(sub) = &claims.sub {
                    if sub == &"test".to_string() {
                        return Some(Box::new(User {
                            username: sub.clone(),
                        }));
                    }
                }
            }
        }
        None
    }
}

#[get("/test")]
async fn test_endpoint(user: User) -> impl Responder {
    HttpResponse::Ok().json(user)
}

fn init_middleware(
) -> HttpAuthenticationMiddleware<BearerAuthenticationExtractor<DefaultJwt>, AllEndpointsMatcher> {
    let user_details_service = JwtUserDetailsServiceImpl {};

    let authentication_provider = JwtAuthenticationProvider::new(Box::new(user_details_service));

    let provider_manager = ProviderManager::new(vec![Box::new(authentication_provider)]);

    let jwt_decoder = SimpleTokenDecoder {};

    let authentication_extractor = BearerAuthenticationExtractor::new(vec![Box::new(jwt_decoder)]);

    let authentication_middleware = HttpAuthenticationMiddleware::new(
        provider_manager,
        Box::new(authentication_extractor),
        Box::new(AllEndpointsMatcher::new()),
    );

    authentication_middleware
}

#[actix_rt::test]
async fn validate_bearer_auth_succeeds() {
    let claims = DefaultJwt {
        iss: None,
        sub: Some("test".to_string()),
        aud: None,
        exp: Some(10000000000),
        nbf: None,
        iat: None,
        jti: None,
    };
    let key = &EncodingKey::from_secret("secret".as_ref());
    let token = encode(&Header::default(), &claims, key).expect("Token couldn't be encoded");

    let mut service =
        test::init_service(App::new().wrap(init_middleware()).service(test_endpoint)).await;

    let req = test::TestRequest::get()
        .uri("/test")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .to_request();

    match service.call(req).await {
        Ok(mut service_response) => {
            assert!(&service_response.status().is_success());
            assert_eq!(
                User {
                    username: "test".to_string()
                },
                deserialize_json(&mut service_response.take_body()).await
            );
        }
        Err(e) => panic!("Error occurred: {}", e),
    }
}

#[actix_rt::test]
async fn validate_basic_auth_invalid_credentials() {
    let claims = DefaultJwt {
        iss: None,
        sub: Some("unknown".to_string()),
        aud: None,
        exp: Some(10000000000),
        nbf: None,
        iat: None,
        jti: None,
    };
    let token = encode(
        &Header::new(Algorithm::default()),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .expect("Token couldn't be encoded");

    let mut service =
        test::init_service(App::new().wrap(init_middleware()).service(test_endpoint)).await;

    let req = test::TestRequest::get()
        .uri("/test")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .to_request();

    match service.call(req).await {
        Ok(service_response) => panic!("Error expected: {:?}", service_response),
        Err(e) => assert_eq!(
            &AuthenticationError::UsernameNotFound,
            e.as_error().expect("error expected")
        ),
    }
}

async fn deserialize_json<T, B>(body: &mut ResponseBody<B>) -> T
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

#[derive(Clone)]
struct SimpleTokenDecoder {}

impl TokenDecoder<DefaultJwt> for SimpleTokenDecoder {
    fn decode_token(&self, token: &str) -> Result<Box<DefaultJwt>, AuthenticationError> {
        let key = &DecodingKey::from_secret("secret".as_ref());
        match decode::<DefaultJwt>(&token, key, &Validation::default()) {
            Ok(token_data) => Ok(Box::new(token_data.claims)),
            Err(_) => Err(AuthenticationError::InvalidToken),
        }
    }
}
