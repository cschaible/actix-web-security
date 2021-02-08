use downcast_rs::Downcast;
use downcast_rs::impl_downcast;

pub mod attachment;
pub mod request_extension;

pub trait UserDetails: Downcast + UserDetailsClone {}
impl_downcast!(UserDetails);

pub trait UserDetailsClone {
    fn clone_box(&self) -> Box<dyn UserDetails>;
}

impl<U> UserDetailsClone for U
    where U: 'static + UserDetails + Clone {
    fn clone_box(&self) -> Box<dyn UserDetails> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn UserDetails> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}