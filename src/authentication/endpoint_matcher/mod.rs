//! The credentials extraction and authentication can be limited to specific endpoints or applied
//! to all endpoints. A `EndpointMatcher` must be instantiated. There are two default implementations
//! available: `AllEndpointsMatcher` to protect all endpoints and `SpecificUrlsMatcher` to protect
//! the URS with the exact matching URLs. Custom ones can be implemented if the defaults are not
//! applicable for the use-case.

use actix_web::dev::ServiceRequest;

/// An `EndpointMatcher` is an implementation that takes a `actix_web::dev::ServiceRequest` instance and
/// decides whether the request must be authenticated or not.
pub trait EndpointMatcher: Send + Sync {
    /// Checks whether the `actix_web::dev::ServiceRequest` must be authenticated or not.
    /// Returns **true** if the request must be authenticated, **false** otherwise.
    fn do_match(&self, req: &ServiceRequest) -> bool;
}

/// The `AllEndpointsMatcher` protects all endpoints. Valid credentials / token are required for all requests.
#[derive(Clone)]
pub struct AllEndpointsMatcher {}

impl AllEndpointsMatcher {
    pub fn new() -> AllEndpointsMatcher {
        AllEndpointsMatcher {}
    }
}

impl Default for AllEndpointsMatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl EndpointMatcher for AllEndpointsMatcher {
    fn do_match(&self, _req: &ServiceRequest) -> bool {
        true
    }
}

/// The `SpecificUrlsMatcher` can be used to protect endpoints with specific URLs.
/// Endpoints that do not match the given URLS are unprotected.
/// Valid credentials / token are required for all requests.
#[derive(Clone)]
pub struct SpecificUrlsMatcher {
    paths: Vec<String>,
}

impl SpecificUrlsMatcher {
    pub fn new(paths: Vec<String>) -> SpecificUrlsMatcher {
        SpecificUrlsMatcher { paths }
    }
}

impl EndpointMatcher for SpecificUrlsMatcher {
    fn do_match(&self, req: &ServiceRequest) -> bool {
        let request_path = req.uri().path().to_string();
        for path in &self.paths {
            if *path == request_path {
                return true;
            }
        }
        false
    }
}
