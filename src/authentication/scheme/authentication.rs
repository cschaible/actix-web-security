use downcast_rs::impl_downcast;
use downcast_rs::Downcast;

pub trait Authentication: Downcast + Sync + Send {}
impl_downcast!(Authentication);
