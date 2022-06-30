#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;

use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct PhysicUpdateSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    time: Read<'a, GameTime>,
    physix: Write<'a, Physix>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for PhysicUpdateSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // update physix
        data.physix.update(data.time.frame_time);
    }
}
