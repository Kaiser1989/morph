#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use nalgebra_glm::*;
use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct PhysicForceSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    time: Read<'a, GameTime>,

    // write components
    velocity: WriteStorage<'a, Velocity>,
    acceleration: WriteStorage<'a, Acceleration>,

    // read components
    physic: ReadStorage<'a, Physic>,
    dynamic: ReadStorage<'a, Dynamic>,
    gravity: ReadStorage<'a, Gravity>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for PhysicForceSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // update forces
        for (_, _, _, mut velocity, gravity, acceleration) in (
            &data.entities,
            &data.physic,
            &data.dynamic,
            &mut data.velocity.restrict_mut(),
            (&data.gravity).maybe(),
            (&data.acceleration).maybe(),
        )
            .join()
        {
            // apply gravity (if available)
            if let Some(gravity) = gravity {
                let velocity = velocity.get_mut();
                velocity.0 += vec2(0.0, gravity.0) * data.time.frame_time;
            }

            // apply acceleration (if available)
            if let Some(acceleration) = acceleration {
                let velocity = velocity.get_mut();
                velocity.0 += acceleration.0 * data.time.frame_time;
                velocity.1 += acceleration.1 * data.time.frame_time;
            }
        }

        // clear acceleration
        data.acceleration.clear();
    }
}
