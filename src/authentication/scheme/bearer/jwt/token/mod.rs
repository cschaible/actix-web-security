use downcast_rs::impl_downcast;
use downcast_rs::Downcast;

pub mod decoder;

pub trait Claims: Downcast + Sync + Send {}
impl_downcast!(Claims);
