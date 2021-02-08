use downcast_rs::Downcast;
use downcast_rs::impl_downcast;

pub trait Authentication: Downcast + Sync + Send {}
impl_downcast!(Authentication);