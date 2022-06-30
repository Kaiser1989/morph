//////////////////////////////////////////////////
// Using

use nalgebra_glm::*;
use specs::prelude::*;
use specs::Component;

//////////////////////////////////////////////////
// Definition

#[derive(Debug, Default, Clone, Copy)]
pub struct Bubble;

#[derive(Debug, Default, Clone, Copy)]
pub struct Water;

#[derive(Debug, Default, Clone, Copy)]
pub struct Rubber;

#[derive(Debug, Default, Clone, Copy)]
pub struct Metal;

#[derive(Debug, Default, Clone, Copy)]
pub struct Block;

#[derive(Debug, Default, Clone, Copy)]
pub struct Spikes;

#[derive(Debug, Default, Clone, Copy)]
pub struct Grid;

#[derive(Debug, Default, Clone, Copy)]
pub struct Court;

#[derive(Debug, Default, Clone, Copy)]
pub struct Portal;

#[derive(Debug, Default, Clone, Copy)]
pub struct Breakable(pub u32); // (group)

#[derive(Debug, Default, Clone, Copy)]
pub struct Accelerator(pub Vec2); // (force)

#[derive(Debug, Default, Clone, Copy)]
pub struct Burst;

#[derive(Debug, Default, Clone, Copy)]
pub struct Slow;

#[derive(Debug, Default, Clone, Copy)]
pub struct Contact(pub Vec2);

#[derive(Debug, Default, Clone, Copy)]
pub struct Finish;

#[derive(Debug, Default, Clone, Copy)]
pub struct Outside;

#[derive(Debug, Default, Clone, Copy)]
pub struct Broken;

#[derive(Debug, Default, Clone, Copy)]
pub struct Blink;

#[derive(Debug, Default, Clone, Copy)]
pub struct Squeeze;

#[derive(Debug, Default, Clone, Copy)]
pub struct Surprise;

//////////////////////////////////////////////////
// Implementation

impl Contact {
    pub fn new(normal: Vec2) -> Contact {
        Contact(normal)
    }

    pub fn empty() -> Contact {
        Contact(vec2(0.0, 0.0))
    }
}

impl Breakable {
    pub fn new(group: u32) -> Breakable {
        Breakable(group)
    }
}

impl Accelerator {
    pub fn new(force: Vec2) -> Accelerator {
        Accelerator(force)
    }
}

//////////////////////////////////////////////////
// Trait Implementation

impl Component for Bubble {
    type Storage = FlaggedStorage<Self, NullStorage<Self>>;
}

impl Component for Water {
    type Storage = FlaggedStorage<Self, NullStorage<Self>>;
}

impl Component for Rubber {
    type Storage = FlaggedStorage<Self, NullStorage<Self>>;
}

impl Component for Metal {
    type Storage = FlaggedStorage<Self, NullStorage<Self>>;
}

impl Component for Block {
    type Storage = NullStorage<Self>;
}

impl Component for Spikes {
    type Storage = NullStorage<Self>;
}

impl Component for Grid {
    type Storage = NullStorage<Self>;
}

impl Component for Court {
    type Storage = NullStorage<Self>;
}

impl Component for Portal {
    type Storage = NullStorage<Self>;
}

impl Component for Breakable {
    type Storage = HashMapStorage<Self>;
}

impl Component for Accelerator {
    type Storage = HashMapStorage<Self>;
}

impl Component for Burst {
    type Storage = FlaggedStorage<Self, NullStorage<Self>>;
}

impl Component for Slow {
    type Storage = FlaggedStorage<Self, NullStorage<Self>>;
}

impl Component for Contact {
    type Storage = FlaggedStorage<Self, HashMapStorage<Self>>;
}

impl Component for Finish {
    type Storage = FlaggedStorage<Self, NullStorage<Self>>;
}

impl Component for Outside {
    type Storage = NullStorage<Self>;
}

impl Component for Broken {
    type Storage = FlaggedStorage<Self, NullStorage<Self>>;
}

impl Component for Blink {
    type Storage = NullStorage<Self>;
}

impl Component for Squeeze {
    type Storage = NullStorage<Self>;
}

impl Component for Surprise {
    type Storage = NullStorage<Self>;
}
