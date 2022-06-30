#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use nalgebra_glm::*;
use shrev::ReaderId;
use specs::prelude::*;

use crate::game::config::*;
use crate::game::ecs::component::*;
use crate::game::ecs::event::*;
use crate::game::ecs::resource::*;
use crate::game::resource::{ComponentTracker, Events};

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct MorphSystem {
    reader: Option<ReaderId<SceneEvent>>,
    bubble_tracker: ComponentTracker<Bubble>,
    water_tracker: ComponentTracker<Water>,
    rubber_tracker: ComponentTracker<Rubber>,
    metal_tracker: ComponentTracker<Metal>,
    //restricted_tracker: ComponentTracker<Restricted>,
    //attracted_tracker: ComponentTracker<Attracted>,
    bursted_tracker: ComponentTracker<Bursted>,
}

#[derive(SystemData)]
pub struct MorphSystemData<'a> {
    entities: Entities<'a>,
    actors: Read<'a, Actors>,

    //linear_movement: WriteStorage<'a, LinearMovement>,
    //springed_follow: WriteStorage<'a, SpringedFollow>,
    morph: ReadStorage<'a, Morph>,
    bubble: ReadStorage<'a, Bubble>,
    water: ReadStorage<'a, Water>,
    rubber: ReadStorage<'a, Rubber>,
    metal: ReadStorage<'a, Metal>,

    //restricted: ReadStorage<'a, Restricted>,
    //attracted: ReadStorage<'a, Attracted>,
    bursted: ReadStorage<'a, Bursted>,
    outside: ReadStorage<'a, Outside>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for MorphSystem {
    type SystemData = MorphSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // update trackers
        self.bubble_tracker.update(&data.bubble);
        self.water_tracker.update(&data.water);
        self.rubber_tracker.update(&data.rubber);
        self.metal_tracker.update(&data.metal);
        //self.restricted_tracker.update(&data.restricted);
        //self.attracted_tracker.update(&data.attracted);
        self.bursted_tracker.update(&data.bursted);

        // update morph change
        for (entity, ..) in (
            &data.entities,
            self.bubble_tracker.inserted()
                | self.bubble_tracker.removed()
                | self.water_tracker.inserted()
                | self.water_tracker.removed()
                | self.rubber_tracker.inserted()
                | self.rubber_tracker.removed()
                | self.metal_tracker.inserted()
                | self.metal_tracker.removed(),
        )
            .join()
        {
            // TODO:
            println!("changed");
        }

        // // adding restriction
        // for (entity, ..) in (
        //     &data.entities,
        //     &data.morph,
        //     self.restricted_tracker.inserted(),
        // )
        //     .join()
        // {
        //     if let Some(linear_movement) = data.linear_movement.get_mut(entity) {
        //         linear_movement.max = CONFIG.physic_grid_max_velocity;
        //     }
        // }

        // // removing restriction
        // for (entity, ..) in (
        //     &data.entities,
        //     &data.bubble,
        //     self.restricted_tracker.removed(),
        // )
        //     .join()
        // {
        //     if let Some(linear_movement) = data.linear_movement.get_mut(entity) {
        //         linear_movement.max = MorphState::Bubble.max_linear_velocity();
        //     }
        // }
        // for (entity, ..) in (
        //     &data.entities,
        //     &data.water,
        //     self.restricted_tracker.removed(),
        // )
        //     .join()
        // {
        //     if let Some(linear_movement) = data.linear_movement.get_mut(entity) {
        //         linear_movement.max = MorphState::Water.max_linear_velocity();
        //     }
        // }

        // // adding devoured
        // for (entity, ..) in (
        //     &data.entities,
        //     &data.morph,
        //     self.devoured_tracker.inserted(),
        // )
        //     .join()
        // {
        //     data.springed_follow.insert(
        //         entity,
        //         SpringedFollow {
        //             target: data.actors.portal.unwrap(),
        //             stiffness: 35.0,
        //             damping: 5.0,
        //         },
        //     );
        // }

        // adding bursted
        for (entity, ..) in (&data.entities, &data.morph, self.bursted_tracker.inserted()).join() {
            //
        }

        // // check events
        // if *data.phase == SystemPhase::Events {
        //     for event in data.events.read_opt(&mut self.reader).into_iter() {
        //         match event {

        //             // Input
        //             LevelEvent::InputMorph(morph_state) => {
        //                 // only check events if morph is available
        //                 if let Some((entity, morph)) = (&data.entities, &mut data.morphs).join().next() {
        //                     if let Some(remaining) = morph.try_morph(morph_state) {
        //                         data.events.write(LevelEvent::MorphChange(entity, morph_state, remaining));
        //                         data.events.write(LevelEvent::CreateMorphEffect(data.entities.create(), entity));
        //                     }
        //                 }
        //             },

        //             // Collisions
        //             LevelEvent::CollisionMorphBlock(morph_entity, _, _)  => {
        //                 let handle = data.physic_handles.get(morph_entity).unwrap();
        //                 if length2(&data.physix.linear_velocity(handle)) > 10.0 {
        //                     data.events.write(LevelEvent::MorphSqueeze(morph_entity, 2));
        //                 }
        //             },
        //             LevelEvent::CollisionMorphBreakable(morph_entity, object_entity, normal) => {
        //                 let handle = data.physic_handles.get(morph_entity).unwrap();
        //                 if length2(&data.physix.linear_velocity(handle)) > 10.0 {
        //                     data.events.write(LevelEvent::MorphSqueeze(morph_entity, 2));
        //                 }
        //                 let morph = data.morphs.get(morph_entity).unwrap();
        //                 if morph.state == MorphState::Rubber || morph.state == MorphState::Metal {
        //                     let kinematic_energy = data.physix.kinematic_energy(handle, normal);
        //                     if kinematic_energy > CONFIG.physic_breakable_energy {
        //                         // emit morph break event
        //                         data.events.write(LevelEvent::MorphBreak(morph_entity, normal));
        //                         // emit object break events
        //                         let impulse = ((kinematic_energy - CONFIG.physic_breakable_energy) / 0.5 / 50.0).min(5.0);
        //                         let breakable = data.breakables.get(object_entity).unwrap();
        //                         for entity in (&data.entities, &data.breakables).join().filter(|(_, b)| b.group == breakable.group).map(|(entity, _)| entity).collect::<Vec<Entity>>() {
        //                             data.events.write(LevelEvent::ObjectBreak(entity, normal, impulse));
        //                         }
        //                     }
        //                 }
        //             },
        //             LevelEvent::IntersectionMorphAccelerator(morph_entity, object_entity) => {
        //                 let morph = data.morphs.get(morph_entity).unwrap();
        //                 let accelerator = data.accelerators.get(object_entity).unwrap();
        //                 if accelerator.morph[morph.state] {
        //                     data.events.write(LevelEvent::MorphAccelerate(morph_entity, accelerator.direction));
        //                     data.events.write(LevelEvent::MorphSurprise(morph_entity, 2));
        //                 }
        //             },
        //             LevelEvent::IntersectionMorphGrid(morph_entity, _) => {
        //                 let morph = data.morphs.get(morph_entity).unwrap();
        //                 data.events.write(LevelEvent::MorphEnterGrid(morph_entity, morph.state));
        //                 data.events.write(LevelEvent::MorphSqueeze(morph_entity, 2));
        //             },
        //             LevelEvent::DisjointMorphGrid(morph_entity, _) => {
        //                 let morph = data.morphs.get(morph_entity).unwrap();
        //                 data.events.write(LevelEvent::MorphLeaveGrid(morph_entity, morph.state));
        //             },
        //             LevelEvent::IntersectionMorphSpike(morph_entity, _) => {
        //                 let morph = data.morphs.get(morph_entity).unwrap();
        //                 if morph.active {
        //                     match morph.state {
        //                         MorphState::Bubble => data.events.write(LevelEvent::MorphBubbleBurst(morph_entity)),
        //                         MorphState::Rubber => data.events.write(LevelEvent::MorphRubberBurst(morph_entity)),
        //                         _ => (),
        //                     }
        //                     data.events.write_delayed(LevelEvent::Failure, 1.5);
        //                 }
        //             },
        //             LevelEvent::IntersectionMorphTarget(morph_entity, object_entity) => {
        //                 let morph = data.morphs.get(morph_entity).unwrap();
        //                 data.events.write(LevelEvent::MorphSurprise(morph_entity, 2));
        //                 if morph.active {
        //                     data.events.write(LevelEvent::MorphTarget(morph_entity, object_entity));
        //                     data.events.write_delayed(LevelEvent::Success, 2.0);
        //                 }
        //             },
        //             LevelEvent::DisjointMorphCourt(morph_entity, _) => {
        //                 let morph = data.morphs.get(morph_entity).unwrap();
        //                 if morph.active {
        //                     data.events.write(LevelEvent::MorphOutside(morph_entity));
        //                     data.events.write_delayed(LevelEvent::Failure, 0.5);
        //                 }
        //             },

        //             // morph actions
        //             LevelEvent::MorphOutside(morph_entity) |
        //             LevelEvent::MorphTarget(morph_entity, _) |
        //             LevelEvent::MorphBubbleBurst(morph_entity) |
        //             LevelEvent::MorphRubberBurst(morph_entity) => {
        //                 let morph = data.morphs.get_mut(morph_entity).unwrap();
        //                 morph.active = false;
        //             }

        //             // ignore
        //             _ => {}
        //         }
        //     }
        // }

        // // update
        // if *data.phase == SystemPhase::Update {
        //     // update morph stuff
        //     if let Some((morph, entity, handle)) = (&data.morphs, &data.entities, &data.physic_handles).join().next() {
        //         if morph.active {
        //             if length2(&data.physix.linear_velocity(handle)) > 24.0 {
        //                 data.events.write(LevelEvent::MorphSurprise(entity, 1));
        //             } else if ((data.time.1 * 10.0).floor() as i32) % 30 == 0 {
        //                 data.events.write(LevelEvent::MorphBlink(entity, 1));
        //             }
        //         }
        //     }
        // }
    }
}

//////////////////////////////////////////////////
// Implementation

impl MorphSystem {
    pub fn init(&mut self, world: &mut World) {
        self.reader = Some(world.get_mut::<Events<SceneEvent>>().unwrap().register());
        self.bubble_tracker.init(world);
        self.water_tracker.init(world);
        self.rubber_tracker.init(world);
        self.metal_tracker.init(world);
        //self.restricted_tracker.init(world);
        //self.attracted_tracker.init(world);
        self.bursted_tracker.init(world);
    }
}
