use std::{io, marker::PhantomData, path::Path};
use thiserror::Error;

use downcast_rs::{Downcast, impl_downcast};

#[derive(Error, Debug)]
pub enum Error
{
    #[error("asset not loadable")]
    NotLoadable,

    #[error("asset not saveable")]
    NotSaveable,

    #[error("asset IO error: {0}")]
    IoError(io::Error),

    #[error("asset Json error: {0}")]
    JsonError(serde_json::Error)
}

impl From<io::Error> for Error
{
    fn from(err: io::Error) -> Error
    {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error
{
    fn from(err: serde_json::Error) -> Error
    {
        Error::JsonError(err)
    }
}

pub trait Asset: Downcast
{
    fn load(&mut self, _path: &Path) -> Result<(), Error>
    {
        Err(Error::NotLoadable)
    }
    fn save(&mut self, _path: &Path) -> Result<(), Error>
    {
        Err(Error::NotSaveable)
    }
}
impl_downcast!(Asset);

#[derive(Clone)]
pub struct Handle<T: Asset>
{
    pub index: u32,
    pub generation: u32,
    pub marker: PhantomData<T>
}

