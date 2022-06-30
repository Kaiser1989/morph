//////////////////////////////////////////////////
// Using

use std::ops::{Add, Mul};

use nalgebra_glm::*;
use specs::prelude::*;

use crate::game::ecs::component::Animatable;

//////////////////////////////////////////////////
// Definition

#[derive(Debug, Default, Clone, Copy)]
pub struct Physic;

#[derive(Debug, Default, Clone, Copy)]
pub struct Dynamic;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Position(pub Vec2);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rotation(pub f32);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Velocity(pub Vec2, pub f32); // (linear, angular)

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct VelocityLimit(pub f32, pub f32); // (linear, angular)

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct VelocityDamping(pub f32, pub f32); // (linear, angular)

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Mass(pub f32, pub f32); // (linear, angular)

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Acceleration(pub Vec2, pub f32); // (linear, angular)

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Gravity(pub f32); // y

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Material(pub f32, pub f32); // (restitution, friction)

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Ball(f32),
    Rect(Vec2),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Collision {
    pub group: usize,
    pub with: Vec<usize>, // TODO: Maybe CollisionGroupHelper for bitwise or
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Sensor {
    pub group: usize,
    pub with: Vec<usize>, // TODO: Maybe CollisionGroupHelper for bitwise or
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Follow(pub Entity);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FollowLag(pub f32); // (lag)

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FollowSpring(pub f32, pub f32); // (stiffness, damping)

//////////////////////////////////////////////////
// Implementation

impl Position {
    pub fn new(pos: Vec2) -> Position {
        Position(pos)
    }
}

impl Rotation {
    pub fn new(rot: f32) -> Rotation {
        Rotation(rot)
    }
}

impl Velocity {
    pub fn new(linear: Vec2, angular: f32) -> Velocity {
        Velocity(linear, angular)
    }
}

impl VelocityLimit {
    pub fn new(linear: f32, angular: f32) -> VelocityLimit {
        VelocityLimit(linear, angular)
    }
}

impl VelocityDamping {
    pub fn new(linear: f32, angular: f32) -> VelocityDamping {
        VelocityDamping(linear, angular)
    }
}

impl Mass {
    pub fn new(linear: f32, angular: f32) -> Mass {
        Mass(linear, angular)
    }
}

impl Acceleration {
    pub fn new(linear: Vec2, angular: f32) -> Acceleration {
        Acceleration(linear, angular)
    }
}

impl Gravity {
    pub fn new(y: f32) -> Gravity {
        Gravity(y)
    }
}

impl Material {
    pub fn new(restitution: f32, friction: f32) -> Material {
        Material(restitution, friction)
    }
}

impl Shape {
    pub fn size(&self) -> Vec2 {
        match self {
            Shape::Ball(radius) => vec2(*radius, *radius),
            Shape::Rect(size) => *size,
        }
    }
}

impl Collision {
    pub fn new(group: usize, with: Vec<usize>) -> Collision {
        Collision { group, with }
    }
}

impl Sensor {
    pub fn new(group: usize, with: Vec<usize>) -> Sensor {
        Sensor { group, with }
    }
}

impl Follow {
    pub fn new(entity: Entity) -> Follow {
        Follow(entity)
    }
}

impl FollowLag {
    pub fn new(lag: f32) -> FollowLag {
        FollowLag(lag)
    }
}

impl FollowSpring {
    pub fn new(stiffness: f32, damping: f32) -> FollowSpring {
        FollowSpring(stiffness, damping)
    }
}

//////////////////////////////////////////////////
// Trait Implementation

impl Component for Physic {
    type Storage = FlaggedStorage<Self, NullStorage<Self>>;
}

impl Component for Dynamic {
    type Storage = FlaggedStorage<Self, NullStorage<Self>>;
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}

impl Animatable for Rotation {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        Rotation::new(lerp_scalar(self.0, other.0, t))
    }
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl Component for VelocityLimit {
    type Storage = DenseVecStorage<Self>;
}

impl Component for VelocityDamping {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Mass {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Gravity {
    type Storage = HashMapStorage<Self>;
}

impl Component for Acceleration {
    type Storage = HashMapStorage<Self>;
}

impl Component for Collision {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Sensor {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Material {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Shape {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Follow {
    type Storage = HashMapStorage<Self>;
}

impl Component for FollowLag {
    type Storage = HashMapStorage<Self>;
}

impl Component for FollowSpring {
    type Storage = HashMapStorage<Self>;
}

impl Add<f32> for Shape {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        match self {
            Shape::Ball(x) => Shape::Ball(x + rhs),
            Shape::Rect(x) => Shape::Rect(x + vec2(rhs, rhs)),
        }
    }
}

impl Mul<f32> for Shape {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Shape::Ball(x) => Shape::Ball(x * rhs),
            Shape::Rect(x) => Shape::Rect(x * rhs),
        }
    }
}

impl Default for Shape {
    fn default() -> Self {
        Shape::Ball(0.0)
    }
}

impl Animatable for Shape {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        match (self, other) {
            (Shape::Ball(x), Shape::Ball(y)) => Shape::Ball(lerp_scalar(*x, *y, t)),
            (Shape::Rect(x), Shape::Rect(y)) => Shape::Rect(lerp(x, y, t)),
            _ => unimplemented!("Cannot change shape"),
        }
    }
}
