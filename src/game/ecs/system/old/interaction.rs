#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use shrev::ReaderId;
use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::event::*;
use crate::game::ecs::resource::*;
use crate::game::resource::Events;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct InteractionSystem {
    reader: Option<ReaderId<SceneEvent>>,
}

#[derive(SystemData)]
pub struct InteractionSystemData<'a> {
    entities: Entities<'a>,
    physix: Read<'a, Physix>,

    dynamic: WriteStorage<'a, Dynamic>,
    follow: WriteStorage<'a, Follow>,
    follow_spring: WriteStorage<'a, FollowSpring>,

    //restricted: WriteStorage<'a, Restricted>,
    bursted: WriteStorage<'a, Bursted>,
    //attracted: WriteStorage<'a, Attracted>,
    outside: WriteStorage<'a, Outside>,

    morph: ReadStorage<'a, Morph>,
    block: ReadStorage<'a, Block>,
    portal: ReadStorage<'a, Portal>,
    court: ReadStorage<'a, Court>,
    spike: ReadStorage<'a, Spikes>,
    grid: ReadStorage<'a, Grid>,
    breakable: ReadStorage<'a, Breakable>,
    accelerator: ReadStorage<'a, Accelerator>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for InteractionSystem {
    type SystemData = InteractionSystemData<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // update morph iteractions
        for (entity, _, _) in (
            &data.entities,
            &data.morph,
            data.physix.interaction_tracker(),
        )
            .join()
        {
            for interaction in data.physix.interactions(&entity) {
                match interaction.action {
                    Action::Sensor(SensorAction::Intersecting) => {
                        // Morph-Portal-Intersection
                        if data.portal.contains(interaction.with) {
                            data.dynamic.remove(entity);
                            data.follow.insert(entity, Follow(interaction.with));
                            data.follow_spring.insert(entity, FollowSpring(35.0, 5.0));
                        }
                        // Morph-Grid-Intersection
                        if data.grid.contains(interaction.with) {
                            //data.restricted.insert(entity, Restriction { max_velocity: });
                        }
                        // Morph-Spikes-Intersection
                        if data.spike.contains(interaction.with) {
                            data.bursted.insert(entity, Bursted);
                        }
                        // Morph-Breakable-Intersection
                        if data.breakable.contains(interaction.with) {
                            // TODO:
                        }
                        // Morph-Accelerator-Intersection
                        if data.accelerator.contains(interaction.with) {
                            // TODO:
                        }
                    }
                    Action::Sensor(SensorAction::Disjoint) => {
                        // Morph-Court-Disjoint
                        if data.court.contains(interaction.with) {
                            data.outside.insert(entity, Outside);
                        }
                        // Morph-Grid-Disjoint
                        if data.grid.contains(interaction.with) {
                            //data.restricted.remove(entity);
                        }
                    }
                    Action::Contact(normal) => {
                        if data.block.contains(interaction.with) {
                            // TODO: Morph-Block-Contact
                        }
                    }
                }
            }
        }

        // // check events
        // if *data.phase == SystemPhase::Events {
        //     for event in data.events.read_opt(&mut self.reader).into_iter() {
        //         match event {
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
    }
}

//////////////////////////////////////////////////
// Implementation

impl InteractionSystem {
    pub fn init(&mut self, world: &mut World) {
        self.reader = Some(world.get_mut::<Events<SceneEvent>>().unwrap().register());
    }
}
