//////////////////////////////////////////////////
// Using

use std::collections::HashMap;

use itertools::Itertools;
use nalgebra::geometry::Isometry2;
use nalgebra_glm::*;
use ncollide2d::pipeline::narrow_phase::Interaction as DefaultInteraction;
use ncollide2d::pipeline::object::CollisionGroups;
use ncollide2d::query::Proximity;
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::algebra::Velocity2;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::material::BasicMaterial;
use nphysics2d::math::{Force, ForceType};
use nphysics2d::object::{
    Body, BodyPartHandle, BodyStatus, Collider as DefaultCollider, ColliderDesc as DefaultColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderHandle, DefaultColliderSet,
    RigidBody as DefaultRigidBody, RigidBodyDesc as DefaultRigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};
use specs::prelude::*;

use crate::game::ecs::component::physic::*;

//////////////////////////////////////////////////
// Definition Alias

pub type MechanicalWorld = DefaultMechanicalWorld<f32>;
pub type GeometricalWorld = DefaultGeometricalWorld<f32>;
pub type BodySet = DefaultBodySet<f32>;
pub type ColliderSet = DefaultColliderSet<f32>;
pub type ConstraintSet = DefaultJointConstraintSet<f32>;
pub type ForceSet = DefaultForceGeneratorSet<f32>;
pub type BodyHandle = DefaultBodyHandle;
pub type ColliderHandle = DefaultColliderHandle;
pub type Collider = DefaultCollider<f32, DefaultBodyHandle>;
pub type RigidBody = DefaultRigidBody<f32>;
pub type RigidBodyDesc = DefaultRigidBodyDesc<f32>;
pub type ColliderDesc = DefaultColliderDesc<f32>;

//////////////////////////////////////////////////
// Definition

#[derive(Debug)]
pub struct PhysixHandle {
    pub body: BodyHandle,
    pub contact: ColliderHandle,
    pub sensor: ColliderHandle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interaction {
    pub with: Entity,
    pub action: Action,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Sensor(SensorAction),
    Contact(Vec2),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorAction {
    Intersecting,
    Disjoint,
}

pub struct Physix {
    mechanical_world: MechanicalWorld,
    geometrical_world: GeometricalWorld,
    body_set: BodySet,
    collider_set: ColliderSet,
    constraint_set: ConstraintSet,
    force_set: ForceSet,
    entities: HashMap<Entity, PhysixHandle>,
    interactions: HashMap<Entity, Vec<Interaction>>,
    interaction_tracker: BitSet,
}
impl Default for Physix {
    fn default() -> Physix {
        Physix::new()
    }
}

//////////////////////////////////////////////////
// Implementation

impl Physix {
    pub fn new() -> Physix {
        Physix {
            mechanical_world: MechanicalWorld::new(vec2(0.0, 0.0)),
            geometrical_world: GeometricalWorld::new(),
            body_set: BodySet::new(),
            collider_set: ColliderSet::new(),
            constraint_set: ConstraintSet::new(),
            force_set: ForceSet::new(),
            entities: HashMap::new(),
            interactions: HashMap::new(),
            interaction_tracker: BitSet::new(),
        }
    }

    // +++ Update +++

    pub fn update(&mut self, elapsed_time: f32) {
        // update all physics with elapsed_time
        self.mechanical_world.set_timestep(elapsed_time);
        self.mechanical_world
            .step(&mut self.geometrical_world, &mut self.body_set, &mut self.collider_set, &mut self.constraint_set, &mut self.force_set);
    }

    pub fn update_interactions(&mut self) {
        self.interactions.clear();
        self.interactions.extend(
            self.geometrical_world
                .interaction_pairs(&self.collider_set, false)
                .filter_map(|(_, collider1, _, collider2, action)| {
                    let entity1 = *collider1.user_data().unwrap().downcast_ref::<Entity>().unwrap();
                    let entity2 = *collider2.user_data().unwrap().downcast_ref::<Entity>().unwrap();
                    match action {
                        DefaultInteraction::Proximity(_, prox) => match prox {
                            Proximity::Intersecting => Some(Action::Sensor(SensorAction::Intersecting)),
                            Proximity::WithinMargin => Some(Action::Sensor(SensorAction::Intersecting)),
                            Proximity::Disjoint => Some(Action::Sensor(SensorAction::Disjoint)),
                        },
                        DefaultInteraction::Contact(_, manifold) if manifold.len() > 0 => manifold.deepest_contact().map(|dc| dc.contact.normal.into_inner()).map(|normal| Action::Contact(normal)),
                        _ => None,
                    }
                    .map(|a| (entity1, entity2, a))
                })
                // duplicate interactions (e1, e2) => (e2, e1)
                .flat_map(|(e1, e2, action)| vec![(e1, Interaction { with: e2, action }), (e2, Interaction { with: e1, action })].into_iter())
                // sort by entity
                .sorted_by_key(|t| t.0)
                // group by entity to generate hashmap
                .chunk_by(|(entity, _)| *entity)
                .into_iter()
                // resolve groups
                .map(|(entity, interactions)| (entity, interactions.map(|(_, i)| i).collect())),
        );

        // update interaction tracker
        self.interaction_tracker.clear();
        for entity in self.interactions.keys() {
            self.interaction_tracker.add(entity.id());
        }
    }

    // +++ Insert & Remove +++

    pub fn insert(&mut self, entity: Entity, position: &Position) {
        // init body
        let body_desc = RigidBodyDesc::new().status(BodyStatus::Static).translation(position.0).status(BodyStatus::Static).user_data(entity);
        let shape = ShapeHandle::new(Ball::new(0.0));
        let contact_desc = ColliderDesc::new(shape.clone()).collision_groups(CollisionGroups::empty()).user_data(entity);
        let sensor_desc = ColliderDesc::new(shape.clone()).collision_groups(CollisionGroups::empty()).sensor(true).user_data(entity);
        // only add to body set, leave colliders unintialized
        let body = self.body_set.insert(body_desc.build());
        let contact = self.collider_set.insert(contact_desc.build(BodyPartHandle(body, 0)));
        let sensor = self.collider_set.insert(sensor_desc.build(BodyPartHandle(body, 0)));
        // create handle
        self.entities.insert(entity, PhysixHandle { body, contact, sensor });
    }

    pub fn remove(&mut self, entity: &Entity) {
        if let Some(handle) = self.entities.remove(entity) {
            self.collider_set.remove(handle.contact);
            self.collider_set.remove(handle.sensor);
            self.body_set.remove(handle.body);
        }
    }

    // +++ Collision +++

    pub fn interaction_tracker(&self) -> &BitSet {
        &self.interaction_tracker
    }

    pub fn interactions(&self, entity: &Entity) -> &[Interaction] {
        if let Some(interactions) = self.interactions.get(entity) {
            interactions
        } else {
            &[]
        }
    }

    // +++ Components +++

    pub fn position(&self, entity: &Entity) -> Position {
        let body = self.body(entity);
        Position(body.position().translation.vector)
    }

    pub fn update_position(&mut self, entity: &Entity, position: &Position) {
        let body = self.body_mut(entity);
        body.set_position(Isometry2::new(position.0, body.position().rotation.angle()));
    }

    pub fn rotation(&self, entity: &Entity) -> Rotation {
        let body = self.body(entity);
        Rotation::new(body.position().rotation.angle())
    }

    pub fn update_rotation(&mut self, entity: &Entity, rotation: &Rotation) {
        let body = self.body_mut(entity);
        body.set_position(Isometry2::new(body.position().translation.vector, rotation.0));
    }

    pub fn update_dynamic(&mut self, entity: &Entity, dynamic: Option<&Dynamic>) {
        let body = self.body_mut(entity);
        body.set_status(if dynamic.is_some() { BodyStatus::Dynamic } else { BodyStatus::Static });
    }

    pub fn velocity(&self, entity: &Entity) -> Velocity {
        let body = self.body(entity);
        let velocity = body.velocity();
        Velocity(velocity.linear, velocity.angular)
    }

    pub fn update_velocity(&mut self, entity: &Entity, velocity: Option<&Velocity>) {
        let body = self.body_mut(entity);
        if let Some(velocity) = velocity {
            body.set_velocity(Velocity2::new(velocity.0, velocity.1));
        } else {
            body.set_velocity(Velocity2::new(vec2(0.0, 0.0), 0.0));
        }
    }

    pub fn update_velocity_limit(&mut self, entity: &Entity, velocity_limit: Option<&VelocityLimit>) {
        let body = self.body_mut(entity);
        if let Some(velocity_limit) = velocity_limit {
            body.set_max_linear_velocity(velocity_limit.0);
            body.set_max_angular_velocity(velocity_limit.1);
        } else {
            body.set_max_linear_velocity(0.0);
            body.set_max_angular_velocity(0.0);
        }
    }

    pub fn update_velocity_damping(&mut self, entity: &Entity, velocity_damping: Option<&VelocityDamping>) {
        let body = self.body_mut(entity);
        if let Some(velocity_damping) = velocity_damping {
            body.set_linear_damping(velocity_damping.0);
            body.set_angular_damping(velocity_damping.1);
        } else {
            body.set_linear_damping(0.0);
            body.set_angular_damping(0.0);
        }
    }

    pub fn update_acceleration(&mut self, entity: &Entity, acceleration: Option<&Acceleration>) {
        if let Some(acceleration) = acceleration {
            let body = self.body_mut(entity);
            body.apply_force(0, &Force::new(acceleration.0, acceleration.1), ForceType::AccelerationChange, true);
        }
    }

    pub fn update_gravity(&mut self, entity: &Entity, gravity: Option<&Gravity>) {
        if let Some(gravity) = gravity {
            self.apply_force_acc(entity, vec2(0.0, gravity.0), 0.0);
        }
    }

    pub fn update_mass(&mut self, entity: &Entity, mass: Option<&Mass>) {
        let body = self.body_mut(entity);
        if let Some(mass) = mass {
            body.set_mass(mass.0);
            body.set_angular_inertia(mass.1);
        } else {
            body.set_mass(0.0);
            body.set_angular_inertia(0.0);
        }
    }

    pub fn update_shape(&mut self, entity: &Entity, shape: Option<&Shape>) {
        let shape = match shape {
            Some(Shape::Ball(radius)) => ShapeHandle::new(Ball::new(*radius)),
            Some(Shape::Rect(size)) => ShapeHandle::new(Cuboid::new(*size)),
            None => ShapeHandle::new(Ball::new(0.0)),
        };
        self.contact_mut(entity).set_shape(shape.clone());
        self.sensor_mut(entity).set_shape(shape.clone());
    }

    pub fn update_material(&mut self, entity: &Entity, material: Option<&Material>) {
        if let Some(material) = material {
            *self.contact_mut(entity).material_mut().downcast_mut().unwrap() = BasicMaterial::new(material.0, material.1);
        } else {
            *self.contact_mut(entity).material_mut().downcast_mut().unwrap() = BasicMaterial::default();
        }
    }

    pub fn update_collision(&mut self, entity: &Entity, collision: Option<&Collision>) {
        if let Some(collision) = collision {
            self.contact_mut(entity)
                .set_collision_groups(CollisionGroups::new().with_membership(&[collision.group]).with_whitelist(&collision.with));
        } else {
            self.contact_mut(entity).set_collision_groups(CollisionGroups::empty());
        }
    }

    pub fn update_sensor(&mut self, entity: &Entity, sensor: Option<&Sensor>) {
        if let Some(sensor) = sensor {
            self.sensor_mut(entity)
                .set_collision_groups(CollisionGroups::new().with_membership(&[sensor.group]).with_whitelist(&sensor.with));
        } else {
            self.sensor_mut(entity).set_collision_groups(CollisionGroups::empty());
        }
    }

    pub fn apply_force_acc(&mut self, entity: &Entity, linear: Vec2, angular: f32) {
        let body = self.body_mut(entity);
        body.apply_force(0, &Force::new(linear, angular), ForceType::AccelerationChange, true);
    }

    pub fn apply_force_vel(&mut self, entity: &Entity, linear: Vec2, angular: f32) {
        let body = self.body_mut(entity);
        body.apply_force(0, &Force::new(linear, angular), ForceType::VelocityChange, true);
    }

    // #[inline]
    // pub fn enable(&mut self, handle: &PhysixHandle, enabled: bool) {
    //     if enabled {
    //         self.body_mut(handle).enable_all_translations();
    //         self.body_mut(handle).enable_all_rotations();
    //     } else {
    //         self.body_mut(handle).disable_all_translations();
    //         self.body_mut(handle).disable_all_rotations();
    //     }
    // }

    // #[inline]
    // pub fn change_status(&mut self, handle: &PhysixHandle, dynamic: bool) {
    //     let status = if dynamic { BodyStatus::Dynamic } else { BodyStatus::Static };
    //     self.body_mut(handle).set_status(status);
    // }

    // #[inline]
    // pub fn kinematic_energy(&self, handle: &PhysixHandle, normal: Vec2) -> f32 {
    //     let normal_velocity = normal * dot(&normal, &self.linear_velocity(handle));
    //     0.5 * self.mass(handle) * length(&normal_velocity) // fake one, square is shit
    // }

    // #[inline]
    // pub fn reduce_kinetatic_enery(&mut self, handle: &PhysixHandle, normal: Vec2, energy: f32) {
    //     let reduction_vec = normal * (energy / 0.5 / self.mass(handle));
    //     let velocity = self.linear_velocity(handle);
    //     self.change_linear_velocity(handle, velocity - reduction_vec);
    // }

    // #[inline]
    // pub fn change_dynamic(&mut self, handle: &PhysixHandle, dynamic: Option<Dynamic>) {
    //     let body = self.body_mut(handle);
    //     if let Some(dynamic) = dynamic.as_ref() {
    //         body.set_status(BodyStatus::Dynamic);
    //         body.set_mass(dynamic.mass);
    //         body.set_max_linear_velocity(dynamic.max_linear_velocity);
    //         body.set_max_angular_velocity(dynamic.max_angular_velocity);
    //         body.set_angular_inertia(dynamic.angular_inertia);
    //         body.set_linear_damping(dynamic.linear_damping);
    //         body.set_angular_damping(dynamic.angular_damping);
    //     } else {
    //         body.set_status(BodyStatus::Static);
    //     }
    // }

    // #[inline]
    // pub fn change_gravity(&mut self, handle: &PhysixHandle, enable_gravity: bool) {
    //     self.body_mut(handle).enable_gravity(enable_gravity);
    // }

    // +++ Internal +++

    #[inline]
    fn body(&self, entity: &Entity) -> &RigidBody {
        let handle = self.entities.get(entity).expect("Entity not found");
        self.body_set.rigid_body(handle.body).expect("Body not found")
    }

    #[inline]
    fn body_mut(&mut self, entity: &Entity) -> &mut RigidBody {
        let handle = self.entities.get(entity).expect("Entity not found");
        self.body_set.rigid_body_mut(handle.body).expect("Body not found")
    }

    #[inline]
    fn contact_mut(&mut self, entity: &Entity) -> &mut Collider {
        let handle = self.entities.get(entity).expect("Entity not found");
        self.collider_set.get_mut(handle.contact).expect("Collider not found")
    }

    #[inline]
    fn sensor_mut(&mut self, entity: &Entity) -> &mut Collider {
        let handle = self.entities.get(entity).expect("Entity not found");
        self.collider_set.get_mut(handle.sensor).expect("Sensor not found")
    }
}
