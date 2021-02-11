use std::collections::HashMap;

use actix_web::dev::HttpResponseBuilder;
use actix_web::http::{header, StatusCode};
use actix_web::{error, HttpResponse};
use once_cell::sync::Lazy;

use crate::authentication::error::error_type::AuthenticationError;

// Status code must be 100 <= code <= 1000
static AUTH_ERROR_STATUS_CODE_MAPPING: Lazy<HashMap<AuthenticationError, u16>> = Lazy::new(|| {
    let mut error_codes: HashMap<AuthenticationError, u16> = HashMap::new();
    add_env_error_code(AuthenticationError::InvalidAuthentication, &mut error_codes);
    add_env_error_code(AuthenticationError::InvalidToken, &mut error_codes);
    add_env_error_code(
        AuthenticationError::InvalidAuthorizationHeader,
        &mut error_codes,
    );
    add_env_error_code(AuthenticationError::UsernameNotFound, &mut error_codes);
    add_env_error_code(
        AuthenticationError::AuthorizationHeaderNotSet,
        &mut error_codes,
    );
    error_codes
});
static AUTH_ERROR_MESSAGE_MAPPING: Lazy<HashMap<AuthenticationError, String>> = Lazy::new(|| {
    let mut error_messages: HashMap<AuthenticationError, String> = HashMap::new();
    add_env_error_message(
        AuthenticationError::InvalidAuthentication,
        "invalid authentication type".to_string(),
        &mut error_messages,
    );
    add_env_error_message(
        AuthenticationError::InvalidToken,
        "access denied".to_string(),
        &mut error_messages,
    );
    add_env_error_message(
        AuthenticationError::InvalidAuthorizationHeader,
        "invalid authorization header".to_string(),
        &mut error_messages,
    );
    add_env_error_message(
        AuthenticationError::UsernameNotFound,
        "access denied".to_string(),
        &mut error_messages,
    );
    add_env_error_message(
        AuthenticationError::AuthorizationHeaderNotSet,
        "authorization header not set".to_string(),
        &mut error_messages,
    );
    error_messages
});
static AUTH_ERROR_CONTENT_TYPE: Lazy<String> =
    Lazy::new(|| match std::env::var("AUTH_ERROR_CONTENT_TYPE") {
        Ok(content_type) => content_type,
        _ => "text/html; charset=utf-8".to_string(),
    });

fn add_env_error_code(
    error: AuthenticationError,
    error_codes: &mut HashMap<AuthenticationError, u16>,
) {
    match std::env::var(format!("{}_code", error.to_string())) {
        Ok(code) => error_codes.insert(
            error,
            code.parse::<u16>().expect("Invalid status code mapping"),
        ),
        _ => error_codes.insert(error, 401),
    };
}

fn add_env_error_message(
    error: AuthenticationError,
    default_message: String,
    error_messages: &mut HashMap<AuthenticationError, String>,
) {
    match std::env::var(format!("{}_message", error.to_string())) {
        Ok(message) => error_messages.insert(error, message),
        _ => error_messages.insert(error, default_message),
    };
}

pub fn overwrite_auth_error_status_code(error: AuthenticationError, status_code: u16) {
    assert!((100..=1000).contains(&status_code), "Invalid status code");
    std::env::set_var(
        format!("{}_code", error.to_string()),
        status_code.to_string(),
    );
}

pub fn overwrite_auth_error_message(error: AuthenticationError, message: String) {
    std::env::set_var(format!("{}_message", error.to_string()), message);
}

pub fn set_auth_error_content_type(content_type: String) {
    std::env::set_var("AUTH_ERROR_CONTENT_TYPE".to_string(), content_type);
}

impl error::ResponseError for AuthenticationError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AuthenticationError::InvalidAuthentication => {
                dynamic_status_code(&AuthenticationError::InvalidAuthentication)
            }
            AuthenticationError::AuthorizationHeaderNotSet => {
                dynamic_status_code(&AuthenticationError::AuthorizationHeaderNotSet)
            }
            AuthenticationError::InvalidAuthorizationHeader => {
                dynamic_status_code(&AuthenticationError::InvalidAuthorizationHeader)
            }
            AuthenticationError::UsernameNotFound => {
                dynamic_status_code(&AuthenticationError::UsernameNotFound)
            }
            AuthenticationError::InvalidToken => {
                dynamic_status_code(&AuthenticationError::InvalidToken)
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, AUTH_ERROR_CONTENT_TYPE.to_string())
            .body(dynamic_error_message(self))
    }
}

fn dynamic_status_code(error: &AuthenticationError) -> StatusCode {
    StatusCode::from_u16(
        *AUTH_ERROR_STATUS_CODE_MAPPING
            .get(error)
            .expect("Status code mapping missing"),
    )
    .expect("Invalid status code mapping found")
}

fn dynamic_error_message(error: &AuthenticationError) -> String {
    AUTH_ERROR_MESSAGE_MAPPING
        .get(error)
        .expect("Error message mapping missing")
        .clone()
}
