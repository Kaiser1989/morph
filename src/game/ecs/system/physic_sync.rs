#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;
use crate::game::resource::ComponentTracker;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct PhysicSyncSystem {
    physic_tracker: ComponentTracker<Physic>,
    dynamic_tracker: ComponentTracker<Dynamic>,
}

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    physix: Write<'a, Physix>,

    // read components
    physic: ReadStorage<'a, Physic>,
    dynamic: ReadStorage<'a, Dynamic>,
    position: ReadStorage<'a, Position>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for PhysicSyncSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
        self.physic_tracker.setup(res);
        self.dynamic_tracker.setup(res);
    }

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
    }
}
