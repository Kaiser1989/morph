#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use nalgebra_glm::*;
use specs::prelude::*;

use crate::game::config::*;
use crate::game::ecs::component::*;
use crate::game::ecs::event::*;
use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct InputCameraSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    actors: Read<'a, Actors>,

    // events
    event_scene_start: Option<Read<'a, EventSceneStart>>,
    event_camera_move: Option<Read<'a, EventCameraMove>>,

    // write components
    velocity: WriteStorage<'a, Velocity>,
    follow: WriteStorage<'a, Follow>,
    follow_lag: WriteStorage<'a, FollowLag>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for InputCameraSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // check scene start event
        if let Some(_) = data.event_scene_start {
            let morph_entity = data.actors.morph.unwrap();
            let camera_entity = data.actors.camera.unwrap();
            data.follow.insert(camera_entity, Follow(morph_entity));
            data.follow_lag.insert(camera_entity, FollowLag(CONFIG.level_camera_follow));
        }

        // check camera move event
        if let Some(camera_move) = data.event_camera_move {
            let camera_entity = data.actors.camera.unwrap();
            let velocity = data.velocity.get_mut(camera_entity).unwrap();
            velocity.0 += vec2(camera_move.0.x, -camera_move.0.y) * CONFIG.level_camera_speed;
        };
    }
}
