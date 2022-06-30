//////////////////////////////////////////////////
// Using

use std::marker::PhantomData;

use specs::prelude::*;

use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

#[derive(Debug, Default, Clone, Copy)]
pub struct Lifetime(pub f32);

#[derive(Debug, Default, Clone, Copy)]
pub struct Insert<C>
where
    C: Component + Send + Sync,
{
    pub lifetime: f32,
    pub component: Option<C>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Remove<C>
where
    C: Component + Send + Sync,
{
    pub lifetime: f32,
    pub component: PhantomData<C>,
}

//////////////////////////////////////////////////
// Implementation

impl Lifetime {
    pub fn new(time: &GameTime, duration: f32) -> Lifetime {
        Lifetime(time.all_time + duration)
    }
}

impl<C> Insert<C>
where
    C: Component + Send + Sync,
{
    pub fn new(component: C, time: &GameTime, duration: f32) -> Insert<C> {
        Insert {
            lifetime: time.all_time + duration,
            component: Some(component),
        }
    }
}

impl<C> Remove<C>
where
    C: Component + Send + Sync,
{
    pub fn new(time: &GameTime, duration: f32) -> Remove<C> {
        Remove {
            lifetime: time.all_time + duration,
            component: PhantomData::default(),
        }
    }
}

//////////////////////////////////////////////////
// Trait Implementation

impl Component for Lifetime {
    type Storage = HashMapStorage<Self>;
}

impl<C> Component for Insert<C>
where
    C: Component + Send + Sync,
{
    type Storage = HashMapStorage<Self>;
}

impl<C> Component for Remove<C>
where
    C: Component + Send + Sync,
{
    type Storage = HashMapStorage<Self>;
}
