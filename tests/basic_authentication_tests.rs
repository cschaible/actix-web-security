use actix_web::dev::Service;
use actix_web::http::header;
use actix_web::{get, test, App, HttpResponse, Responder};
use async_trait::async_trait;

use actix_web_security::authentication::endpoint_matcher::AllEndpointsMatcher;
use actix_web_security::authentication::error::error_type::AuthenticationError;
use actix_web_security::authentication::middleware::HttpAuthenticationMiddleware;
use actix_web_security::authentication::scheme::basic::authentication_provider::BasicAuthenticationProvider;
use actix_web_security::authentication::scheme::basic::header_extractor::BasicAuthenticationExtractor;
use actix_web_security::authentication::scheme::basic::user_details_service::BasicUserDetailsService;
use actix_web_security::authentication::ProviderManager;
use actix_web_security::user_details::UserDetails;

use common::deserialize_json;
use common::User;

mod common;

#[derive(Clone)]
struct BasicUserDetailsServiceImpl {}

#[async_trait]
impl BasicUserDetailsService for BasicUserDetailsServiceImpl {
    async fn find_user(&self, username: &str, password: &str) -> Option<Box<dyn UserDetails>> {
        if username == "test" && password == "pw" {
            Some(Box::new(User {
                username: username.to_string(),
            }))
        } else {
            None
        }
    }
}

#[get("/test")]
async fn test_endpoint(user: User) -> impl Responder {
    HttpResponse::Ok().json(user)
}

fn init_middleware(
) -> HttpAuthenticationMiddleware<BasicAuthenticationExtractor, AllEndpointsMatcher> {
    let user_details_service = BasicUserDetailsServiceImpl {};

    let authentication_provider = BasicAuthenticationProvider::new(Box::new(user_details_service));

    let provider_manager = ProviderManager::new(vec![Box::new(authentication_provider)]);

    let authentication_extractor = BasicAuthenticationExtractor::new();

    let authentication_middleware = HttpAuthenticationMiddleware::new(
        provider_manager,
        Box::new(authentication_extractor),
        Box::new(AllEndpointsMatcher::new()),
    );

    authentication_middleware
}

#[actix_rt::test]
async fn validate_basic_auth_succeeds() {
    let encoded_credentials = base64::encode("test:pw");

    let mut service =
        test::init_service(App::new().wrap(init_middleware()).service(test_endpoint)).await;

    let req = test::TestRequest::get()
        .uri("/test")
        .header(
            header::AUTHORIZATION,
            format!("Basic {}", encoded_credentials),
        )
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
    let encoded_credentials = base64::encode("unknown:pw");

    let mut service =
        test::init_service(App::new().wrap(init_middleware()).service(test_endpoint)).await;

    let req = test::TestRequest::get()
        .uri("/test")
        .header(
            header::AUTHORIZATION,
            format!("Basic {}", encoded_credentials),
        )
        .to_request();

    match service.call(req).await {
        Ok(service_response) => panic!("Error expected: {:?}", service_response),
        Err(e) => assert_eq!(
            &AuthenticationError::UsernameNotFound,
            e.as_error().expect("error expected")
        ),
    }
}
