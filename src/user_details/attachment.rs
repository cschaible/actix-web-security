//! Utility trait to attach a `UserDetails` object to the request context.

use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;

use crate::user_details::UserDetails;

/// A helper trait to attch a boxed `UserDetails` object to the request context.
pub trait UserDetailsRequestAttachmentHelper {
    fn attach(&self, user_details: Box<dyn UserDetails>);
}

impl UserDetailsRequestAttachmentHelper for ServiceRequest {
    fn attach(&self, user_details: Box<dyn UserDetails>) {
        self.extensions_mut().insert(user_details);
    }
}
