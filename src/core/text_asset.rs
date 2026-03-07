use std::{path::Path, fs::{read_to_string, write}};

use uuid::Uuid;

use crate::core::asset::Asset;

#[derive(Debug, Default)]
pub struct TextAsset
{
    uuid: Uuid,
    data: String,
}

impl TextAsset
{
    pub fn new(uuid: Uuid) -> Self
    {
        Self{
            uuid,
            ..Self::default()
        }
    }

    pub fn set_data(&mut self, data: String) { self.data = data; }
    pub fn data(&self) -> &str { &self.data }
}

impl Asset for TextAsset
{
    fn load(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>>
    {
        self.data = read_to_string(path)?;
        Ok(())
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>>
    {
        write(path, &mut self.data)?;
        Ok(())
    }

    fn uuid(&self) -> &Uuid
    {
        &self.uuid
    }
}

