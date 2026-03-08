use std::{any::{Any, TypeId}, collections::HashMap, path::PathBuf};

use uuid::Uuid;

use crate::core::{asset::{self, Asset, Handle}, asset_registry::Registry};

struct Slot<T: Asset>
{
    generation: u32,
    asset: Option<T>
}

type Assets<T> = Vec<Slot<T>>;

struct Storage 
{
    assets: Box<dyn Any>, 
    free_list: Vec<u32>
}

pub struct Context
{
    storages: HashMap<TypeId, Storage>,

    registry: Registry, 
    registry_path: PathBuf,

    uuid_map: HashMap<Uuid, (TypeId, u32, u32)>
}

impl Context
{
    pub fn new(registry_path: PathBuf) -> Self
    {
        let mut registry = Registry::new();

        let _ = registry.load(&registry_path);

        Self{
            storages: HashMap::new(),

            registry,
            registry_path,

            uuid_map: HashMap::new()
        }
    }

    pub fn save_registry(&mut self) -> Result<(), asset::Error>
    {
        self.registry.save(&self.registry_path)
    }

    pub fn add<T: Asset + 'static>(&mut self, name: String, asset: T) -> Handle<T>
    {
        let type_id = TypeId::of::<T>();

        let storage = self.storages
            .entry(type_id)
            .or_insert_with(|| Storage{ assets: Box::new(Assets::<T>::new()), free_list: Vec::new() });

        let slots = storage.assets.downcast_mut::<Assets::<T>>().unwrap();

        let (index, generation) = if let Some(free_index) = storage.free_list.pop()
        {
            let slot = &mut slots[free_index as usize];

            slot.generation += 1;
            slot.asset = Some(asset);

            (free_index, slot.generation)
        }
        else 
        {
            let index = slots.len() as u32;
            slots.push(Slot{ generation: 0, asset: Some(asset) });
            (index, 0)
        };

        let uuid = self.registry.add(name);
        self.uuid_map.insert(uuid, (type_id, index, generation));

        Handle{
            index,
            generation,
            marker: std::marker::PhantomData
        }
    } 

    pub fn remove<T: Asset + 'static>(&mut self, handle: &Handle<T>) -> Option<T>
    {
        let type_id = TypeId::of::<T>();

        let storage = self.storages.get_mut(&type_id)?;
        let slots = storage.assets.downcast_mut::<Assets::<T>>().unwrap();
        let slot = slots.get_mut(handle.index as usize)?;

        if handle.generation != slot.generation
        {
            return None;
        }

        slot.asset.take()
    }

    pub fn get<T: Asset + 'static>(&self, handle: &Handle<T>) -> Option<&T>
    {
        let type_id = TypeId::of::<T>();

        let storage = self.storages.get(&type_id)?;
        let slots = storage.assets.downcast_ref::<Assets::<T>>().unwrap();
        let slot = slots.get(handle.index as usize)?;

        if handle.generation != slot.generation
        {
            return None;
        }

        slot.asset.as_ref()
    }

    pub fn get_mut<T: Asset + 'static>(&mut self, handle: &Handle<T>) -> Option<&mut T>
    {
        let type_id = TypeId::of::<T>();

        let storage = self.storages.get_mut(&type_id)?;
        let slots = storage.assets.downcast_mut::<Assets::<T>>().unwrap();
        let slot = slots.get_mut(handle.index as usize)?;

        if handle.generation != slot.generation
        {
            return None;
        }

        slot.asset.as_mut()
    }

    pub fn get_handle<T: Asset + 'static>(&self, uuid: &Uuid) -> Option<Handle<T>>
    {
        let (type_id, index, generation) = self.uuid_map.get(uuid)?;
        if *type_id != TypeId::of::<T>()
        {
            return None;
        }

        Some(Handle{
            index: *index,
            generation: *generation,
            marker: std::marker::PhantomData
        })
    }

    pub fn get_uuid(&self, name: &str) -> Option<&Uuid> 
    {
        self.registry.get(name)
    }

    pub fn registry(&self) -> &Registry { &self.registry }
}

