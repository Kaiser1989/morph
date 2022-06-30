#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;

use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct PhysicInteractionSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    physix: Write<'a, Physix>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for PhysicInteractionSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // update interactions
        data.physix.update_interactions();
    }
}
