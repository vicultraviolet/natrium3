use std::{path::Path, fs::{read_to_string, write}};

use crate::asset::{Asset, Error};

#[derive(Debug, Default)]
pub struct Text
{
    data: String,
}

impl Text
{
    pub fn new() -> Self
    {
        Self{
            ..Self::default()
        }
    }

    pub fn set_data(&mut self, data: String) { self.data = data; }
    pub fn data(&self) -> &str { &self.data }
}

impl Asset for Text
{
    fn load(&mut self, path: &Path) -> Result<(), Error>
    {
        self.data = read_to_string(path)?;
        Ok(())
    }

    fn save(&mut self, path: &Path) -> Result<(), Error>
    {
        write(path, &mut self.data)?;
        Ok(())
    }
}

