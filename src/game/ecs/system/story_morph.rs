#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use nalgebra_glm::*;
use specs::prelude::*;

use crate::game::config::*;
use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;
use crate::game::resource::ComponentTracker;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct StoryMorphSystem {
    bubble_tracker: ComponentTracker<Bubble>,
    water_tracker: ComponentTracker<Water>,
    rubber_tracker: ComponentTracker<Rubber>,
    metal_tracker: ComponentTracker<Metal>,
    slow_tracker: ComponentTracker<Slow>,
    finish_tracker: ComponentTracker<Finish>,
    burst_tracker: ComponentTracker<Burst>,
}

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    actors: Read<'a, Actors>,

    // write components
    dynamic: WriteStorage<'a, Dynamic>,
    rotation: WriteStorage<'a, Rotation>,
    velocity: WriteStorage<'a, Velocity>,
    velocity_limit: WriteStorage<'a, VelocityLimit>,
    follow: WriteStorage<'a, Follow>,
    follow_spring: WriteStorage<'a, FollowSpring>,

    // read components
    bubble: ReadStorage<'a, Bubble>,
    water: ReadStorage<'a, Water>,
    rubber: ReadStorage<'a, Rubber>,
    metal: ReadStorage<'a, Metal>,
    slow: ReadStorage<'a, Slow>,
    finish: ReadStorage<'a, Finish>,
    burst: ReadStorage<'a, Burst>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for StoryMorphSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
        self.bubble_tracker.setup(res);
        self.water_tracker.setup(res);
        self.rubber_tracker.setup(res);
        self.metal_tracker.setup(res);
        self.slow_tracker.setup(res);
        self.finish_tracker.setup(res);
        self.burst_tracker.setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // update trackers
        self.bubble_tracker.update(&data.bubble);
        self.water_tracker.update(&data.water);
        self.rubber_tracker.update(&data.rubber);
        self.metal_tracker.update(&data.metal);
        self.slow_tracker.update(&data.slow);
        self.finish_tracker.update(&data.finish);
        self.burst_tracker.update(&data.burst);

        // calculate masks
        let morph_mask = data.bubble.mask() | data.water.mask() | data.rubber.mask() | data.metal.mask();

        // handle slow insertion
        for (_, velocity_limit, _) in (&data.entities, &mut data.velocity_limit, &morph_mask & self.slow_tracker.inserted()).join() {
            velocity_limit.0 = CONFIG.physic_grid_max_velocity;
        }

        // handle slow removing
        for (entity, velocity_limit, _) in (&data.entities, &mut data.velocity_limit, &morph_mask & self.slow_tracker.removed()).join() {
            if data.bubble.contains(entity) {
                *velocity_limit = MorphState::Bubble.velocity_limit();
            }
            if data.water.contains(entity) {
                *velocity_limit = MorphState::Water.velocity_limit();
            }
            if data.rubber.contains(entity) {
                *velocity_limit = MorphState::Rubber.velocity_limit();
            }
            if data.metal.contains(entity) {
                *velocity_limit = MorphState::Metal.velocity_limit();
            }
        }

        // handle burst insertion
        for (entity, _) in (&data.entities, &morph_mask & self.burst_tracker.inserted()).join() {
            // bubble burst
            if data.bubble.contains(entity) {
                data.dynamic.remove(entity);
            }

            // rubber burst
            if data.rubber.contains(entity) {
                let rotation = data.rotation.get_mut(entity).unwrap();
                rotation.0 = 0.0;
                let velocity = data.velocity.get_mut(entity).unwrap();
                velocity.0 = vec2(0.0, 0.0);
                velocity.1 = 0.0;
            }
        }

        // handle finish insertion
        for (entity, _) in (&data.entities, &morph_mask & self.finish_tracker.inserted()).join() {
            data.dynamic.remove(entity);
            data.follow.insert(entity, Follow(data.actors.portal.unwrap()));
            data.follow_spring.insert(entity, FollowSpring(35.0, 5.0));
        }
    }
}
