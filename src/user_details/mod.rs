use downcast_rs::impl_downcast;
use downcast_rs::Downcast;

pub mod attachment;
pub mod request_extension;

pub trait UserDetails: Downcast + UserDetailsClone {}
impl_downcast!(UserDetails);

pub trait UserDetailsClone {
    fn clone_box(&self) -> Box<dyn UserDetails>;
}

// this is an interesting pattern, i have never seen it so far.
// I don't quite get why you don't make UserDetails require Clone, please tell me :D

impl<U> UserDetailsClone for U
where
    U: 'static + UserDetails + Clone,
    // I don't know wny the static lifetime is necessary here, maybe i will get it when I see an example
{
    fn clone_box(&self) -> Box<dyn UserDetails> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn UserDetails> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
