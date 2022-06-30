#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use smallvec::*;
use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;
use crate::game::resource::ComponentTracker;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct StoryObjectAnimationSystem {
    broken_tracker: ComponentTracker<Broken>,
}

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    time: Read<'a, GameTime>,

    // write components
    opacity: WriteStorage<'a, Opacity>,
    opacity_anim_insert: WriteStorage<'a, Insert<Animation<Opacity>>>,

    // read components
    broken: ReadStorage<'a, Broken>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for StoryObjectAnimationSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
        self.broken_tracker.setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // update trackers
        self.broken_tracker.update(&data.broken);

        // breakable broken animation
        for (entity, _) in (&data.entities, self.broken_tracker.inserted()).join() {
            data.opacity.insert(entity, Opacity::new(1.0));
            data.opacity_anim_insert.insert(
                entity,
                Insert::new(
                    Animation::new(smallvec![Opacity::new(1.0), Opacity::new(0.0)], 0.25),
                    &data.time,
                    1.25,
                ),
            );
        }
    }
}
