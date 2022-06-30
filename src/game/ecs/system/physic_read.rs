#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct PhysicReadSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    physix: Write<'a, Physix>,

    // read components
    physic: ReadStorage<'a, Physic>,
    position: ReadStorage<'a, Position>,
    rotation: ReadStorage<'a, Rotation>,
    velocity: ReadStorage<'a, Velocity>,
    velocity_limit: ReadStorage<'a, VelocityLimit>,
    velocity_damping: ReadStorage<'a, VelocityDamping>,
    mass: ReadStorage<'a, Mass>,
    material: ReadStorage<'a, Material>,
    shape: ReadStorage<'a, Shape>,
    collision: ReadStorage<'a, Collision>,
    sensor: ReadStorage<'a, Sensor>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for PhysicReadSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // update ECS => Physix
        for (entity, _, position, rotation, velocity, velocity_limit, velocity_damping, mass, shape, material, collision, sensor) in (
            &data.entities,
            &data.physic,
            &data.position,
            &data.rotation,
            (&data.velocity).maybe(),
            (&data.velocity_limit).maybe(),
            (&data.velocity_damping).maybe(),
            (&data.mass).maybe(),
            (&data.shape).maybe(),
            (&data.material).maybe(),
            (&data.collision).maybe(),
            (&data.sensor).maybe(),
        )
            .join()
        {
            // update physix data
            data.physix.update_position(&entity, position);
            data.physix.update_rotation(&entity, rotation);
            data.physix.update_velocity(&entity, velocity);
            data.physix.update_velocity_limit(&entity, velocity_limit);
            data.physix.update_velocity_damping(&entity, velocity_damping);
            data.physix.update_mass(&entity, mass);
            data.physix.update_shape(&entity, shape);
            data.physix.update_material(&entity, material);
            data.physix.update_collision(&entity, collision);
            data.physix.update_sensor(&entity, sensor);
        }
    }
}
