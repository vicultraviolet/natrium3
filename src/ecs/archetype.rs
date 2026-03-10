use std::{any::TypeId, collections::HashMap};

use downcast_rs::{Downcast, impl_downcast};

use crate::ecs::{component::Component, entity::Entity};

trait ContainerTrait: Downcast {}
impl_downcast!(ContainerTrait);

struct Container<T: Component>
{
    components: Vec<T>
}

impl<T: Component> Container<T>
{
    fn new() -> Self
    {
        Self{
            components: Vec::new()
        }
    }
}

impl<T: Component> ContainerTrait for Container<T> {}

pub struct Archetype
{
    signature: Vec<TypeId>,
    containers: Vec<Box<dyn ContainerTrait>>,
    entities: Vec<Entity>
}

impl Archetype
{
    pub fn new(mut signature: Vec<TypeId>) -> Self
    {
        signature.sort();
        Self{
            signature,
            containers: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn get_component<T: Component>(&self, entity_index: usize) -> Option<&T>
    {
        if entity_index >= self.entities.len()
        {
            return None;
        }

        match self.get_type_index(TypeId::of::<T>())
        {
            Some(index) =>
                self.containers[index]
                .downcast_ref::<Container<T>>()
                .unwrap()
                .components
                .get(entity_index),
            None => None
        }
    }

    fn get_type_index(&self, type_id: TypeId) -> Option<usize>
    {
        self.signature.binary_search(&type_id).ok()
    }
}

macro_rules! impl_push_n
{
    () => {};

    ($n:tt: $($ty:ident),*) =>
    {
        paste::item!
        {
            pub fn [<push $n>]<$($ty: Component),*>(
                &mut self,
                e: Entity,
                c: ($($ty,)*)
            ) {
                if self.containers.is_empty()
                {
                    let mut type_to_container = HashMap::new();
                    $(
                        type_to_container.insert(
                            TypeId::of::<$ty>(),
                            Box::new(Container::<$ty>::new()) as Box<dyn ContainerTrait>
                        );
                    )*

                    for &type_id in &self.signature 
                    {
                        let container = type_to_container.remove(&type_id).unwrap();

                        self.containers.push(container);
                    }
                }

                #[allow(non_snake_case)]
                let ($($ty,)*) = c;

                $(
                    let type_id = TypeId::of::<$ty>();
                    let index = self.get_type_index(type_id).unwrap();

                    self.containers[index]
                        .downcast_mut::<Container<$ty>>()
                        .unwrap()
                        .components
                        .push($ty);
                )*

                self.entities.push(e);
            }
        }
    };
}

impl Archetype 
{
    impl_push_n!(1:  A);
    impl_push_n!(2:  A, B);
    impl_push_n!(3:  A, B, C);
    impl_push_n!(4:  A, B, C, D);
    impl_push_n!(5:  A, B, C, D, E);
    impl_push_n!(6:  A, B, C, D, E, F);
    impl_push_n!(7:  A, B, C, D, E, F, G);
    impl_push_n!(8:  A, B, C, D, E, F, G, H);
    impl_push_n!(9:  A, B, C, D, E, F, G, H, I);
    impl_push_n!(10: A, B, C, D, E, F, G, H, I, J);
    impl_push_n!(11: A, B, C, D, E, F, G, H, I, J, K);
    impl_push_n!(12: A, B, C, D, E, F, G, H, I, J, K, L);
    impl_push_n!(13: A, B, C, D, E, F, G, H, I, J, K, L, M);
    impl_push_n!(14: A, B, C, D, E, F, G, H, I, J, K, L, M, N);
    impl_push_n!(15: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
    impl_push_n!(16: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
    impl_push_n!(17: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
    impl_push_n!(18: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
    impl_push_n!(19: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
    impl_push_n!(20: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
    impl_push_n!(21: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
    impl_push_n!(22: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
    impl_push_n!(23: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
    impl_push_n!(24: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
    impl_push_n!(25: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
    impl_push_n!(26: A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
}

