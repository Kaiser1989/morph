//////////////////////////////////////////////////
// Using

use smallvec::SmallVec;
use specs::prelude::*;

//////////////////////////////////////////////////
// const

const MAX_ANIMATION_ITEMS: usize = 8;

//////////////////////////////////////////////////
// Traits

pub trait Animatable: Default {
    fn interpolate(&self, other: &Self, t: f32) -> Self;
}

//////////////////////////////////////////////////
// Definition

#[derive(Debug, Clone, Copy)]
pub enum AnimationKind {
    Single,
    Repeat,
}

#[derive(Debug, Default, Clone)]
pub struct Animation<C>
where
    C: Animatable + Component + Send + Sync,
{
    pub duration: f32,
    pub current: f32,
    pub frames: SmallVec<[C; MAX_ANIMATION_ITEMS]>,
    pub kind: AnimationKind,
}

//////////////////////////////////////////////////
// Implementation

impl<C> Animation<C>
where
    C: Animatable + Component + Send + Sync,
{
    pub fn new(frames: SmallVec<[C; MAX_ANIMATION_ITEMS]>, duration: f32) -> Animation<C> {
        Animation {
            frames,
            duration,
            current: 0.0,
            kind: Default::default(),
        }
    }

    pub fn with_kind(frames: SmallVec<[C; MAX_ANIMATION_ITEMS]>, duration: f32, kind: AnimationKind) -> Animation<C> {
        Animation { frames, duration, current: 0.0, kind }
    }
}

//////////////////////////////////////////////////
// Trait Implementation

impl Default for AnimationKind {
    fn default() -> Self {
        AnimationKind::Single
    }
}

impl<C> Component for Animation<C>
where
    C: Animatable + Component + Send + Sync,
{
    type Storage = HashMapStorage<Self>;
}
