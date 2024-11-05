#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use nalgebra_glm::*;
use rand::distributions::{Distribution, Uniform};
use specs::prelude::*;
use specs::storage::GenericWriteStorage;

use crate::game::config::*;
use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;
use crate::game::resource::ComponentTracker;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct StoryObjectSystem {
    contact_tracker: ComponentTracker<Contact>,
}

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    time: Read<'a, GameTime>,
    config: Read<'a, Config>,

    // write components
    dynamic: WriteStorage<'a, Dynamic>,
    rotation: WriteStorage<'a, Rotation>,
    velocity: WriteStorage<'a, Velocity>,
    velocity_damping: WriteStorage<'a, VelocityDamping>,
    velocity_limit: WriteStorage<'a, VelocityLimit>,
    mass: WriteStorage<'a, Mass>,
    gravity: WriteStorage<'a, Gravity>,
    material: WriteStorage<'a, Material>,
    collision: WriteStorage<'a, Collision>,
    broken: WriteStorage<'a, Broken>,
    lifetime: WriteStorage<'a, Lifetime>,

    // read components
    contact: ReadStorage<'a, Contact>,
    breakable: ReadStorage<'a, Breakable>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for StoryObjectSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
        self.contact_tracker.setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        let config = data.config;

        // update trackers
        self.contact_tracker.update(&data.contact);

        // check breakables
        for (entity, breakable, _) in (&data.entities, &data.breakable, self.contact_tracker.inserted()).join() {
            let broken = data.broken.get(entity).is_some();
            let mut contact = *data.contact.get(entity).unwrap();
            let impulse_length = length(&contact.0);
            if !broken && impulse_length > config.physic_break_impulse {
                // find all group entities
                for (entity, _) in (&data.entities, &data.breakable).join().filter(|(_, b)| b.0 == breakable.0) {
                    // random angular rotation
                    let mut rng = rand::thread_rng();
                    let rotation = data.rotation.get_mut(entity).unwrap();
                    let rotation_offset = Uniform::new_inclusive(-0.2, 0.2).sample(&mut rng);
                    rotation.0 += rotation_offset;

                    // random velocity
                    // (linear)
                    contact.0 *= config.physic_break_impulse / impulse_length;
                    let velocity = data.velocity.get_mut_or_default(entity).unwrap();
                    velocity.0 = rotate_vec2(
                        &(contact.0 * Uniform::new_inclusive(0.8, 1.0).sample(&mut rng) * 0.2),
                        Uniform::new_inclusive(-0.2, 0.2).sample(&mut rng),
                    );
                    // (angular)
                    velocity.1 = rotation_offset * Uniform::new_inclusive(5.0, 8.0).sample(&mut rng);

                    // add components
                    data.dynamic.insert(entity, Dynamic);
                    data.mass.insert(entity, Mass::new(5.0, 0.5));
                    data.velocity_damping.insert(entity, VelocityDamping::new(0.1, 0.1));
                    data.velocity_limit.insert(entity, VelocityLimit::new(10.0, 10.0));
                    data.gravity.insert(entity, Gravity::new(-9.81));
                    data.material.insert(entity, Material::new(0.3, 0.5));
                    data.collision.insert(entity, Role::Particle.collision(&config));
                    data.broken.insert(entity, Broken);
                    data.lifetime.insert(entity, Lifetime::new(&data.time, 1.5));
                }
            }
        }
    }
}
