//////////////////////////////////////////////////
// Using

use nalgebra_glm::*;
use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;
use crate::game::fx::TextureSrc;

//////////////////////////////////////////////////
// Definition

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Camera {
    pub zoom: f32,
    pub max_dimension: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Texture(pub TextureSrc);

#[derive(Debug, Clone, Copy, Default)]
pub struct TextureSlot(pub f32);

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Layer {
    pub plane: Plane,
    pub rank: u8,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color(pub Vec4);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Opacity(pub f32);

//////////////////////////////////////////////////
// Implementation

impl Camera {
    pub fn new(zoom: f32, max_dimension: Vec2) -> Camera {
        Camera { zoom, max_dimension }
    }
}

impl Texture {
    pub fn new(source: TextureSrc) -> Texture {
        Texture(source)
    }
}

impl TextureSlot {
    pub fn new(slot: f32) -> TextureSlot {
        TextureSlot(slot)
    }
}

impl Layer {
    pub fn new(plane: Plane, rank: u8) -> Layer {
        Layer { plane, rank }
    }
}

impl Color {
    pub fn new(color: Vec4) -> Color {
        Color(color)
    }
}

impl Opacity {
    pub fn new(a: f32) -> Opacity {
        Opacity(a)
    }
}

//////////////////////////////////////////////////
// Trait Implementation

impl Component for Camera {
    type Storage = HashMapStorage<Self>;
}

impl Component for Texture {
    type Storage = DenseVecStorage<Self>;
}

impl Component for TextureSlot {
    type Storage = DenseVecStorage<Self>;
}

impl Animatable for TextureSlot {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        TextureSlot(lerp_scalar(self.0, other.0, t))
    }
}

impl Component for Layer {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Color {
    type Storage = HashMapStorage<Self>;
}

impl Component for Opacity {
    type Storage = HashMapStorage<Self>;
}

impl Default for Opacity {
    fn default() -> Self {
        Opacity(1.0)
    }
}

impl Animatable for Opacity {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        Opacity(lerp_scalar(self.0, other.0, t))
    }
}
