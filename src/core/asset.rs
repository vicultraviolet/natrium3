use std::{error::Error, path::Path};

use downcast_rs::{Downcast, impl_downcast};
use uuid::Uuid;

pub trait Asset: Downcast
{
    fn load(&mut self, path: &Path) -> Result<(), Box<dyn Error>>;
    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>>;

    fn uuid(&self) -> &Uuid;
}
impl_downcast!(Asset);

