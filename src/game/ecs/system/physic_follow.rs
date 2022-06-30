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
pub struct PhysicFollowSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    time: Read<'a, GameTime>,
    physix: Write<'a, Physix>,

    // read components
    follow: ReadStorage<'a, Follow>,
    follow_lag: ReadStorage<'a, FollowLag>,
    follow_spring: ReadStorage<'a, FollowSpring>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for PhysicFollowSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // update follow (need special treatment)
        for (entity, follow, follow_lag, follow_spring) in (&data.entities, &data.follow, (&data.follow_lag).maybe(), (&data.follow_spring).maybe()).join() {
            // read data from physix
            let target_pos = data.physix.position(&follow.0);
            let mut entity_pos = data.physix.position(&entity);
            let mut entity_vel = data.physix.velocity(&entity);

            // calculate follow
            match (follow_lag, follow_spring) {
                // simple follow, no lag, no spring
                (None, None) => {
                    entity_pos.0 = target_pos.0;
                }
                // lag follow
                (Some(FollowLag(lag)), None) => {
                    entity_pos.0 = lerp(&target_pos.0, &entity_pos.0, *lag);
                }
                // spring follow (has precedence over lag)
                (_, Some(FollowSpring(stiffness, damping))) => {
                    let force = (target_pos.0 - entity_pos.0) * *stiffness;
                    let damping = entity_vel.0 * *damping;
                    entity_vel.0 += (force - damping) * data.time.frame_time;
                    entity_pos.0 += entity_vel.0 * data.time.frame_time;
                }
            };

            // update physix
            data.physix.update_position(&entity, &entity_pos);
            data.physix.update_velocity(&entity, Some(&entity_vel));
        }
    }
}
