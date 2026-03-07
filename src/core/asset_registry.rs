use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::asset::Asset;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AssetRegistry
{
    uuid: Uuid,
    registry: HashMap<String, Uuid>
}

impl AssetRegistry
{
    pub fn new(uuid: Uuid) -> Self
    {
        Self{
            uuid,
            ..Self::default()
        }
    }

    pub fn new_asset<T: Asset>(
        &mut self,
        name: String,
        constructor: impl FnOnce(Uuid) -> T
    ) -> Box<T> 
    {
        let uuid = match self.registry.get(&name)
        {
            Some(&id) => id,
            None => {
                let new_uuid = Uuid::new_v4();
                self.registry.insert(name, new_uuid);
                new_uuid
            }
        };

        Box::new(constructor(uuid))
    }

    pub fn get(&self, name: &str) -> Option<&Uuid>
    {
        self.registry.get(name)
    }
}

impl Asset for AssetRegistry
{
    fn load(&mut self, path: &Path) -> Result<(), Box<dyn Error>>
    {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        *self = serde_json::from_reader(reader)?;

        Ok(())
    }

    fn save(&mut self, path: &Path) -> Result<(), Box<dyn Error>>
    {
        let mut file = File::create(path)?;

        serde_json::to_writer(&mut file, self)?;

        Ok(())
    }

    fn uuid(&self) -> &Uuid { &self.uuid }
}

