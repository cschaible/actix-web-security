use actix_web::dev::ServiceRequest;

pub trait EndpointMatcher: Send + Sync {
    fn do_match(&self, req: &ServiceRequest) -> bool;
}

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
