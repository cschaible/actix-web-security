use std::cell::RefCell;
use std::future::{self, Future, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};

use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;

use crate::authentication::endpoint_matcher::EndpointMatcher;
use crate::authentication::error::error_type::AuthenticationError;
use crate::authentication::scheme::header_extractor::AuthorizationHeaderExtractor;
use crate::authentication::ProviderManager;
use crate::user_details::attachment::UserDetailsRequestAttachmentHelper;

pub struct HttpAuthenticationMiddleware<T, U>
where
    T: AuthorizationHeaderExtractor + Clone,
    U: EndpointMatcher + Clone,
{
    authorization_extractor: Box<T>,
    provider_manager: ProviderManager,
    endpoint_matcher: Box<U>,
}

impl<T: AuthorizationHeaderExtractor + Clone, U: EndpointMatcher + Clone>
    HttpAuthenticationMiddleware<T, U>
{
    pub fn new(
        provider_manager: ProviderManager,
        authorization_extractor: Box<T>,
        endpoint_matcher: Box<U>,
    ) -> HttpAuthenticationMiddleware<T, U> {
        HttpAuthenticationMiddleware {
            authorization_extractor,
            provider_manager,
            endpoint_matcher,
        }
    }
}

impl<S, B, T, U> Transform<S> for HttpAuthenticationMiddleware<T, U>
where
    T: AuthorizationHeaderExtractor + Clone + 'static,
    U: EndpointMatcher + Clone,
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = HttpAuthenticationService<S, T, U>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let service = Rc::new(RefCell::new(service));
        let provider_manager = Arc::new(self.provider_manager.clone());
        let authorization_extractor = Arc::new(self.authorization_extractor.clone());
        let endpoint_matcher = Arc::new(self.endpoint_matcher.clone());
        future::ready(Ok(HttpAuthenticationService {
            service,
            provider_manager,
            authorization_extractor,
            endpoint_matcher,
        }))
    }
}

pub struct HttpAuthenticationService<
    S,
    T: AuthorizationHeaderExtractor + Clone,
    U: EndpointMatcher + Clone,
> {
    service: Rc<RefCell<S>>,
    provider_manager: Arc<ProviderManager>,
    authorization_extractor: Arc<Box<T>>,
    endpoint_matcher: Arc<Box<U>>,
}

impl<S, B, T, U> Service for HttpAuthenticationService<S, T, U>
where
    U: EndpointMatcher + Clone,
    T: AuthorizationHeaderExtractor + Clone + 'static,
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let service = Rc::clone(&self.service);
        let handle_request = self.endpoint_matcher.do_match(&req);

        if handle_request {
            let authorization_extractor = Arc::clone(&self.authorization_extractor);
            let provider_manager = Arc::clone(&self.provider_manager);
            Box::pin(async move {
                let error: Option<AuthenticationError>;

                let extracted_token = authorization_extractor.extract_token(&req.headers()).await;
                match extracted_token {
                    Ok(token) => {
                        let authentication_result = provider_manager.authenticate(&token).await;
                        match authentication_result {
                            Ok(result) => {
                                req.attach(result);
                                error = None;
                            }
                            Err(e) => error = Some(e),
                        };
                    }
                    Err(e) => error = Some(e),
                };

                match error {
                    Some(e) => Err(e.into()),
                    None => {
                        let fut = service.borrow_mut().call(req);
                        let res = fut.await?;

                        Ok(res)
                    }
                }
            })
        } else {
            Box::pin(async move {
                let fut = service.borrow_mut().call(req);
                let res = fut.await?;

                Ok(res)
            })
        }
    }
}
