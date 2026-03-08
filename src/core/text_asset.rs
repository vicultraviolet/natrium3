use std::{path::Path, fs::{read_to_string, write}};

use crate::core::asset::{self, Asset};

#[derive(Debug, Default)]
pub struct TextAsset
{
    data: String,
}

impl TextAsset
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

impl Asset for TextAsset
{
    fn load(&mut self, path: &Path) -> Result<(), asset::Error>
    {
        self.data = read_to_string(path)?;
        Ok(())
    }

    fn save(&mut self, path: &Path) -> Result<(), asset::Error>
    {
        write(path, &mut self.data)?;
        Ok(())
    }
}

