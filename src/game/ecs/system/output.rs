#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct OutputSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,

    // write components
    output: Write<'a, Output>,

    // read components
    bubble: ReadStorage<'a, Bubble>,
    water: ReadStorage<'a, Water>,
    rubber: ReadStorage<'a, Rubber>,
    metal: ReadStorage<'a, Metal>,
    burst: ReadStorage<'a, Burst>,
    finish: ReadStorage<'a, Finish>,
    outside: ReadStorage<'a, Outside>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for OutputSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // calculate masks
        let morph_mask = data.bubble.mask() | data.water.mask() | data.rubber.mask() | data.metal.mask();

        // morph finish => success
        for (_, _, _) in (&data.entities, &data.finish, &morph_mask).join() {
            data.output.success(1.5);
        }

        // morph bursted => failure
        for (_, _, _, _) in (&data.entities, &data.bubble, &data.burst, &morph_mask).join() {
            data.output.failure(1.0);
        }
        for (_, _, _, _) in (&data.entities, &data.rubber, &data.burst, &morph_mask).join() {
            data.output.failure(1.5);
        }

        // morph outside => failure
        for (_, _, _) in (&data.entities, &data.outside, &morph_mask).join() {
            data.output.failure(0.25);
        }
    }
}
