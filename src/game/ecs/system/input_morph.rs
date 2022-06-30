#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::event::*;
use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct InputMorphSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    actors: Read<'a, Actors>,

    // events
    event_scene_start: Option<Read<'a, EventSceneStart>>,
    event_scene_end: Option<Read<'a, EventSceneEnd>>,
    event_morph: Option<Read<'a, EventMorph>>,

    // write components
    bubble: WriteStorage<'a, Bubble>,
    water: WriteStorage<'a, Water>,
    rubber: WriteStorage<'a, Rubber>,
    metal: WriteStorage<'a, Metal>,
    dynamic: WriteStorage<'a, Dynamic>,
    velocity_limit: WriteStorage<'a, VelocityLimit>,
    velocity_damping: WriteStorage<'a, VelocityDamping>,
    gravity: WriteStorage<'a, Gravity>,
    mass: WriteStorage<'a, Mass>,
    collision: WriteStorage<'a, Collision>,
    sensor: WriteStorage<'a, Sensor>,
    material: WriteStorage<'a, Material>,
    shape: WriteStorage<'a, Shape>,
    texture: WriteStorage<'a, Texture>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for InputMorphSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // check scene start event
        if let Some(_) = data.event_scene_start {
            let morph_entity = data.actors.morph.unwrap();
            data.dynamic.insert(morph_entity, Dynamic);
        }

        // check scene end event
        if let Some(_) = data.event_scene_end {
            // TODO:
        }

        // check morph change event
        if let Some(morph_event) = &data.event_morph {
            let morph_entity = data.actors.morph.unwrap();
            let morph_state = morph_event.0;
            // change morph comp
            data.bubble.remove(morph_entity);
            data.water.remove(morph_entity);
            data.rubber.remove(morph_entity);
            data.metal.remove(morph_entity);
            match morph_state {
                MorphState::Bubble => {
                    data.bubble.insert(morph_entity, Bubble);
                }
                MorphState::Water => {
                    data.water.insert(morph_entity, Water);
                }
                MorphState::Rubber => {
                    data.rubber.insert(morph_entity, Rubber);
                }
                MorphState::Metal => {
                    data.metal.insert(morph_entity, Metal);
                }
            }
            // change physics
            data.velocity_limit
                .insert(morph_entity, morph_state.velocity_limit());
            data.velocity_damping
                .insert(morph_entity, morph_state.velocity_damping());
            data.gravity.insert(morph_entity, morph_state.gravity());
            data.mass.insert(morph_entity, morph_state.mass());
            data.collision.insert(morph_entity, morph_state.collision());
            data.sensor.insert(morph_entity, morph_state.sensor());
            data.material.insert(morph_entity, morph_state.material());
            data.shape.insert(morph_entity, morph_state.shape());
            data.texture.insert(morph_entity, morph_state.texture());
        }
    }
}
