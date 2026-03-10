use downcast_rs::{Downcast, impl_downcast};
pub use natrium3_derive::Component;

pub trait Component: 'static + Send + Sync + Downcast {}
impl_downcast!(Component);

