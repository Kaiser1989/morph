#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct PhysicWriteSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    physix: Write<'a, Physix>,

    // write components
    position: WriteStorage<'a, Position>,
    rotation: WriteStorage<'a, Rotation>,
    velocity: WriteStorage<'a, Velocity>,

    // read components
    physic: ReadStorage<'a, Physic>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for PhysicWriteSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // update Physix => ECS
        for (entity, _, mut position, mut rotation, mut velocity) in (
            &data.entities,
            &data.physic,
            (&mut data.position.restrict_mut()).maybe(),
            (&mut data.rotation.restrict_mut()).maybe(),
            (&mut data.velocity.restrict_mut()).maybe(),
        )
            .join()
        {
            // update position component
            if let Some(position) = position.as_mut() {
                let physix_position = data.physix.position(&entity);
                if position.get() != &physix_position {
                    *position.get_mut() = physix_position;
                }
            }

            // update rotation component
            if let Some(rotation) = rotation.as_mut() {
                let physix_rotation = data.physix.rotation(&entity);
                if rotation.get() != &physix_rotation {
                    *rotation.get_mut() = physix_rotation;
                }
            }

            // update velocity component
            if let Some(velocity) = velocity.as_mut() {
                let physix_velocity = data.physix.velocity(&entity);
                if velocity.get() != &physix_velocity {
                    *velocity.get_mut() = physix_velocity;
                }
            }
        }
    }
}
