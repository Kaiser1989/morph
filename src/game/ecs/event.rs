//////////////////////////////////////////////////
// Using

use nalgebra_glm::*;
use specs::prelude::*;

use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

pub struct EventSceneStart;
pub struct EventSceneEnd;

pub struct EventMorph(pub MorphState);
pub struct EventCameraMove(pub Vec2);

pub fn write_event<T>(world: &mut World, event: T)
where
    T: Resource,
{
    world.insert(event);
    world.read_resource::<LazyUpdate>().exec(|world| {
        world.remove::<T>();
    });
}
