use downcast_rs::Downcast;
use downcast_rs::impl_downcast;

pub mod decoder;

pub trait Claims: Downcast + Sync + Send {}
impl_downcast!(Claims);