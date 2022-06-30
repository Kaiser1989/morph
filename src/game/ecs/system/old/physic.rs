#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use nalgebra_glm::*;
use shrev::ReaderId;
use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::event::*;
use crate::game::ecs::resource::*;
use crate::game::resource::{ComponentTracker, Events};

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct PhysicSystem {
    reader: Option<ReaderId<SceneEvent>>,
    physic_tracker: ComponentTracker<Physic>,
    dynamic_tracker: ComponentTracker<Dynamic>,
}

#[derive(SystemData)]
pub struct PhysicSystemData<'a> {
    entities: Entities<'a>,
    time: Read<'a, GameTime>,
    physix: Write<'a, Physix>,

    position: WriteStorage<'a, Position>,
    velocity: WriteStorage<'a, Velocity>,
    gravity: WriteStorage<'a, Gravity>,
    acceleration: WriteStorage<'a, Acceleration>,

    physic: ReadStorage<'a, Physic>,
    dynamic: ReadStorage<'a, Dynamic>,

    velocity_limit: ReadStorage<'a, VelocityLimit>,
    velocity_damping: ReadStorage<'a, VelocityDamping>,

    mass: ReadStorage<'a, Mass>,
    collision: ReadStorage<'a, Collision>,
    sensor: ReadStorage<'a, Sensor>,
    material: ReadStorage<'a, Material>,
    shape: ReadStorage<'a, Shape>,

    follow: ReadStorage<'a, Follow>,
    follow_lag: ReadStorage<'a, FollowLag>,
    follow_spring: ReadStorage<'a, FollowSpring>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for PhysicSystem {
    type SystemData = PhysicSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // update trackers
        self.physic_tracker.update(&data.physic);
        self.dynamic_tracker.update(&data.dynamic);

        // remove entities
        for (entity, _) in (&data.entities, self.physic_tracker.removed()).join() {
            data.physix.remove(&entity);
        }

        // add entities
        for (entity, position, _) in (
            &data.entities,
            &data.position,
            self.physic_tracker.inserted(),
        )
            .join()
        {
            data.physix.insert(entity, position);
        }

        // update physic status
        for (entity, _, _) in (
            &data.entities,
            &data.physic,
            self.dynamic_tracker.inserted() | self.dynamic_tracker.removed(),
        )
            .join()
        {
            let dynamic = data.dynamic.get(entity);
            data.physix.update_dynamic(&entity, dynamic);
        }

        // update ECS => Physix
        for (
            entity,
            _,
            position,
            velocity,
            velocity_limit,
            velocity_damping,
            gravity,
            acceleration,
            mass,
            shape,
            material,
            collision,
            sensor,
        ) in (
            &data.entities,
            &data.physic,
            &data.position,
            (&data.velocity).maybe(),
            (&data.velocity_limit).maybe(),
            (&data.velocity_damping).maybe(),
            (&data.gravity).maybe(),
            (&data.acceleration).maybe(),
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
            data.physix.update_velocity(&entity, velocity);
            data.physix.update_velocity_limit(&entity, velocity_limit);
            data.physix
                .update_velocity_damping(&entity, velocity_damping);
            data.physix.update_acceleration(&entity, acceleration);
            data.physix.update_gravity(&entity, gravity);
            data.physix.update_mass(&entity, mass);
            data.physix.update_shape(&entity, shape);
            data.physix.update_material(&entity, material);
            data.physix.update_collision(&entity, collision);
            data.physix.update_sensor(&entity, sensor);
        }

        // update physix
        data.physix.update(data.time.frame_time);

        // update follow (need special treatment)
        for (entity, follow, follow_lag, follow_spring) in (
            &data.entities,
            &data.follow,
            (&data.follow_lag).maybe(),
            (&data.follow_spring).maybe(),
        )
            .join()
        {
            let target_pos = data.physix.position(&follow.0);
            let mut entity_pos = data.physix.position(&entity);
            let mut entity_vel = data.physix.velocity(&entity);
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
            data.physix.update_position(&entity, &entity_pos);
            data.physix.update_velocity(&entity, Some(&entity_vel));
        }

        // update interactions
        data.physix.update_interactions();

        // update Physix => ECS
        for (entity, _, mut position, mut velocity) in (
            &data.entities,
            &data.physic,
            (&mut data.position.restrict_mut()).maybe(),
            (&mut data.velocity.restrict_mut()).maybe(),
        )
            .join()
        {
            // update position component
            if let Some(position) = position.as_mut() {
                let physix_position = data.physix.position(&entity);
                if position.get_unchecked() != &physix_position {
                    *position.get_mut_unchecked() = physix_position;
                }
            }
            // update velocity component
            if let Some(velocity) = velocity.as_mut() {
                let physix_velocity = data.physix.velocity(&entity);
                if velocity.get_unchecked() != &physix_velocity {
                    *velocity.get_mut_unchecked() = physix_velocity;
                }
            }
        }
        // clear acceleration (as only valid for single frame)
        data.acceleration.clear();

        // // check events
        // if *data.phase == SystemPhase::Events {
        //     for event in data.events.read_opt(&mut self.reader).into_iter() {
        //         match event {

        //             // morph actions
        //             LevelEvent::MorphChange(entity, state, _) => {
        //                 let handle = data.physic_handles.get(entity).unwrap();
        //                 data.physix.change_dynamic(handle, Some(state.dynamic()));
        //                 data.physix.change_material(handle, Some(state.material()));
        //                 data.physix.change_collision(handle, Some(state.collision()));
        //                 if state.inverse_gravity() {
        //                     data.inverse_gravities.insert(entity, InverseGravity::new());
        //                 } else {
        //                     data.inverse_gravities.remove(entity);
        //                 }
        //             },
        //             LevelEvent::MorphTarget(entity, target) => {
        //                 let handle = data.physic_handles.get(entity).unwrap();
        //                 data.physix.change_gravity(handle, false);
        //                 data.follows.insert(entity, Follow::new(target, FollowType::DampedSpring(35.0, 5.0)));
        //             },
        //             LevelEvent::MorphBubbleBurst(entity) => {
        //                 let handle = data.physic_handles.get(entity).unwrap();
        //                 data.physix.enable(handle, false);
        //             },
        //             LevelEvent::MorphRubberBurst(entity) => {
        //                 let handle = data.physic_handles.get(entity).unwrap();
        //                 data.physix.change_velocity(handle, vec2(0.0, 0.0), 0.0);
        //                 data.physix.change_rotation(handle, 0.0);
        //             },
        //             LevelEvent::MorphEnterGrid(entity, _) => {
        //                 let handle = data.physic_handles.get(entity).unwrap();
        //                 data.physix.change_max_linear_velocity(handle, CONFIG.physic_grid_max_velocity);
        //             },
        //             LevelEvent::MorphLeaveGrid(entity, state) => {
        //                 let handle = data.physic_handles.get(entity).unwrap();
        //                 data.physix.change_max_linear_velocity(handle, state.max_linear_velocity());
        //             },
        //             LevelEvent::MorphAccelerate(entity, force) => {
        //                 let handle = data.physic_handles.get(entity).unwrap();
        //                 data.physix.add_force_acc(handle, force, 0.0);
        //             },
        //             LevelEvent::MorphBreak(entity, normal) => {
        //                 let handle = data.physic_handles.get(entity).unwrap();
        //                 data.physix.reduce_kinetatic_enery(handle, normal, CONFIG.physic_breakable_energy);
        //             },

        //             // object actions
        //             LevelEvent::ObjectBreak(entity, normal, impulse) => {
        //                 let mut rng = rand::thread_rng();
        //                 let handle = data.physic_handles.get(entity).unwrap();
        //                 let rotation_random = Uniform::new_inclusive(-0.2, 0.2).sample(&mut rng);
        //                 let rotation = data.physix.rotation(handle) + rotation_random;
        //                 let direction = rotate_vec2(&normal, Uniform::new_inclusive(-0.2, 0.2).sample(&mut rng));
        //                 let linear_velocity = direction * impulse * Uniform::new_inclusive(0.8, 1.0).sample(&mut rng);
        //                 let angular_velocity = rotation_random * Uniform::new_inclusive(5.0, 8.0).sample(&mut rng);
        //                 data.physix.change_rotation(handle, rotation);
        //                 data.physix.change_velocity(handle, linear_velocity, angular_velocity);
        //                 data.physix.change_dynamic(handle, Some(Dynamic::new(5.0, 10.0, 0.1, 0.5, 10.0, 0.1)));
        //                 data.physix.change_collision(handle, Some(Collision::new(CONFIG.physic_group_particle, Role::Particle.collision_group(), Vec::new())));
        //                 data.physix.change_material(handle, Some(Material::new(0.3, 0.5)));
        //             },

        //             // Create entities
        //             LevelEvent::CreateMorphEffect(entity, morph_entity) => {
        //                 data.physic_infos.insert(entity, PhysicInfo::new(vec2(0.0, 0.0), 0.0, vec2(0.0, 0.0), 0.0, Some(Shape::new_ball(CONFIG.level_morph_size * 1.5)), None, None, None));
        //                 data.follows.insert(entity, Follow::new(morph_entity, FollowType::Instant));
        //             },

        //             // ignore
        //             _ => {}
        //         }
        //     };
        // }

        // // update
        // if *data.phase == SystemPhase::Update {
        //     // check if we need to add new entities
        //     insert_new_entities(&mut data);

        //     // check if we need special Gravity
        //     for (_, physic_handle) in (&data.inverse_gravities, &data.physic_handles).join() {
        //         data.physix.add_force_acc(physic_handle, vec2(0.0, 9.81) * 2.0, 0.0);
        //     }

        //     // update physic
        //     data.physix.update(data.time.0);

        //     // get collisions
        //     for (entity1, entity2, collision) in data.physix.collisions() {
        //         // get roles
        //         let (morph_entity, object_entity, object_role) = match (data.roles.get(entity1), data.roles.get(entity2)) {
        //             (Some(Role::Morph), Some(role)) => (entity1, entity2, role),
        //             (Some(role), Some(Role::Morph)) => (entity2, entity1, role),
        //             _ => continue,
        //         };
        //         let morph = data.morphs.get(morph_entity).unwrap();
        //         // check collisions
        //         match (morph.state, object_role, collision) {
        //             // check hitting block
        //             (_, Role::Block, Interaction::Contact(normal)) => {
        //                 data.events.write(LevelEvent::CollisionMorphBlock(morph_entity, object_entity, normal));
        //             },
        //             // Check hitting Breakable (metal, rubber)
        //             (_, Role::Breakable, Interaction::Contact(normal)) => {
        //                 data.events.write(LevelEvent::CollisionMorphBreakable(morph_entity, object_entity, normal));
        //             },
        //             // check hitting target
        //             (_, Role::Target, Interaction::Sensor(SensorAction::Intersecting)) => {
        //                 data.events.write(LevelEvent::IntersectionMorphTarget(morph_entity, object_entity));
        //             },
        //             // Check hitting spikes (rubber, bubble)
        //             (_, Role::Spikes, Interaction::Sensor(SensorAction::Intersecting)) => {
        //                 data.events.write(LevelEvent::IntersectionMorphSpike(morph_entity, object_entity));
        //             },
        //             // check leaving court
        //             (_, Role::Court, Interaction::Sensor(SensorAction::Disjoint)) => {
        //                 data.events.write(LevelEvent::DisjointMorphCourt(morph_entity, object_entity));
        //             },
        //             // Check hitting acceleration field
        //             (_, Role::Accelerator, Interaction::Sensor(SensorAction::Intersecting)) => {
        //                 data.events.write(LevelEvent::IntersectionMorphAccelerator(morph_entity, object_entity));
        //             },
        //             // Entering Grid
        //             (_, Role::Grid, Interaction::Sensor(SensorAction::Intersecting)) => {
        //                 data.events.write(LevelEvent::IntersectionMorphGrid(morph_entity, object_entity));
        //             },
        //             // Leaving Grid
        //             (_, Role::Grid, Interaction::Sensor(SensorAction::Disjoint)) => {
        //                 data.events.write(LevelEvent::DisjointMorphGrid(morph_entity, object_entity));
        //             },
        //             _ => {}
        //         }
        //     }
    }
}

//////////////////////////////////////////////////
// Implementation

impl PhysicSystem {
    pub fn init(&mut self, world: &mut World) {
        self.reader = Some(world.get_mut::<Events<SceneEvent>>().unwrap().register());
        self.physic_tracker.init(world);
        self.dynamic_tracker.init(world);
    }
}
