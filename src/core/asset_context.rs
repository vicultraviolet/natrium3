use std::{collections::HashMap, error::Error, path::PathBuf};

use uuid::Uuid;

use crate::core::{asset::Asset, asset_registry::AssetRegistry};

#[derive(Default)]
pub struct AssetContext
{
    registry: AssetRegistry,
    registry_path: PathBuf,
    assets: HashMap<Uuid, Box<dyn Asset>>
}

impl AssetContext
{
    pub fn new(registry_path: PathBuf) -> Self
    {
        let mut registry = AssetRegistry::default();
        let _ = registry.load(&registry_path);

        Self{
            registry,
            registry_path,
            ..Self::default()
        }
    }

    pub fn save_registry(&mut self) -> Result<(), Box<dyn Error>> { self.registry.save(&self.registry_path) }

    pub fn new_asset<T: Asset + 'static>(
        &mut self,
        name: &str,
        constructor: impl FnOnce(Uuid) -> T
    ) -> &mut T 
    {
        if self.get_typed_by_name::<T>(name).is_some()
        {
            return self.get_typed_mut_by_name(name).unwrap();
        }

        let asset = self.registry.new_asset(String::from(name), constructor);

        self.assets.insert(*asset.uuid(), asset);

        self.get_typed_mut_by_name(name).unwrap()
    }

    pub fn release_asset(&mut self, uuid: &Uuid) -> Option<Box<dyn Asset>>
    {
        self.assets.remove(uuid)
    }

    pub fn get_uuid(&self, name: &str) -> Option<&Uuid>
    {
        self.registry.get(name)
    }

    pub fn get(&self, uuid: &Uuid) -> Option<&dyn Asset>
    {
        if let Some(asset) = self.assets.get(uuid)
        {
            return Some(&**asset);
        }
        None
    }

    pub fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut dyn Asset>
    {
        self.assets.get_mut(uuid).map(|b| &mut **b)
    }

    pub fn get_typed<T: Asset + 'static>(&self, uuid: &Uuid) -> Option<&T>
    {
        self.get(uuid)?.downcast_ref::<T>()
    }

    pub fn get_typed_mut<T: Asset + 'static>(&mut self, uuid: &Uuid) -> Option<&mut T>
    {
        self.get_mut(uuid)?.downcast_mut::<T>()
    }

    pub fn get_by_name(&self, name: &str) -> Option<&dyn Asset>
    {
        let uuid = self.registry.get(name)?;
        self.get(uuid)
    }

    pub fn get_mut_by_name(&mut self, name: &str) -> Option<&mut dyn Asset>
    {
        let uuid = *self.registry.get(name)?;
        self.get_mut(&uuid)
    }

    pub fn get_typed_by_name<T: Asset + 'static>(&self, name: &str) -> Option<&T>
    {
        let uuid = self.registry.get(name)?;
        self.get_typed(uuid)
    }

    pub fn get_typed_mut_by_name<T: Asset + 'static>(&mut self, name: &str) -> Option<&mut T>
    {
        let uuid = *self.registry.get(name)?;
        self.get_typed_mut(&uuid)
    }
}

