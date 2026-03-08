use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::asset::{self, Asset};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Registry
{
    registry: HashMap<String, Uuid>
}

impl Registry
{
    pub fn new() -> Self
    {
        Self{
            ..Self::default()
        }
    }

    pub fn add(&mut self, name: String) -> Uuid
    {
        match self.registry.get(&name)
        {
            Some(&uuid) => uuid,
            None => {
                let uuid = Uuid::new_v4();
                self.registry.insert(name, uuid);
                uuid
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&Uuid>
    {
        self.registry.get(name)
    }
}

impl Asset for Registry
{
    fn load(&mut self, path: &Path) -> Result<(), asset::Error>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        *self = serde_json::from_reader(reader)?;

        Ok(())
    }

    fn save(&mut self, path: &Path) -> Result<(), asset::Error>
    {
        let mut file = File::create(path)?;

        serde_json::to_writer(&mut file, self)?;

        Ok(())
    }
}

