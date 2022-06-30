#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;
use specs::storage::GenericWriteStorage;

use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;
use crate::game::utils::vec::*;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct StoryInteractionSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    physix: Read<'a, Physix>,
    time: Read<'a, GameTime>,

    // write components
    acceleration: WriteStorage<'a, Acceleration>,
    burst: WriteStorage<'a, Burst>,
    slow: WriteStorage<'a, Slow>,
    contact: WriteStorage<'a, Contact>,
    contact_remove: WriteStorage<'a, Remove<Contact>>,
    finish: WriteStorage<'a, Finish>,
    outside: WriteStorage<'a, Outside>,

    // read components
    velocity: ReadStorage<'a, Velocity>,
    mass: ReadStorage<'a, Mass>,
    bubble: ReadStorage<'a, Bubble>,
    water: ReadStorage<'a, Water>,
    rubber: ReadStorage<'a, Rubber>,
    metal: ReadStorage<'a, Metal>,
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

impl<'a> System<'a> for StoryInteractionSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // calculate masks
        let morph_mask = data.bubble.mask() | data.water.mask() | data.rubber.mask() | data.metal.mask();

        // update morph iteractions
        for (entity, _) in (&data.entities, morph_mask & data.physix.interaction_tracker()).join() {
            for interaction in data.physix.interactions(&entity) {
                match interaction.action {
                    Action::Sensor(SensorAction::Intersecting) => {
                        // Morph-Portal-Intersection
                        if data.portal.contains(interaction.with) {
                            data.finish.insert(entity, Finish);
                        }
                        // Morph-Grid-Intersection
                        if data.grid.contains(interaction.with) {
                            data.slow.insert(entity, Slow);
                        }
                        // Morph-Spikes-Intersection
                        if data.spike.contains(interaction.with) {
                            data.burst.insert(entity, Burst);
                        }
                        // Morph-Accelerator-Intersection
                        if data.accelerator.contains(interaction.with) {
                            let accelerator = data.accelerator.get(interaction.with).unwrap();
                            if let Some(acc) = data.acceleration.get_mut_or_default(entity) {
                                acc.0 += accelerator.0;
                            }
                        }
                    }
                    Action::Sensor(SensorAction::Disjoint) => {
                        // Morph-Court-Disjoint
                        if data.court.contains(interaction.with) {
                            data.outside.insert(entity, Outside);
                        }
                        // Morph-Grid-Disjoint
                        if data.grid.contains(interaction.with) {
                            data.slow.remove(entity);
                        }
                    }
                    Action::Contact(normal) => {
                        // Morph-Block-Contact
                        if data.block.contains(interaction.with) {
                            // calculate impulse
                            let velocity = data.velocity.get(entity).unwrap();
                            let mass = data.mass.get(entity).unwrap();
                            let impulse = velocity.0 * mass.0;
                            let normal_impulse = project(&impulse, &normal);
                            data.contact.insert(entity, Contact::new(normal_impulse));
                            data.contact_remove.insert(entity, Remove::new(&data.time, 0.01));
                        }

                        // Morph-Breakable-Contact
                        if data.breakable.contains(interaction.with) {
                            // calculate impulse
                            let velocity = data.velocity.get(entity).unwrap();
                            let mass = data.mass.get(entity).unwrap();
                            let impulse = velocity.0 * mass.0;
                            let normal_impulse = project(&impulse, &normal);
                            data.contact.insert(entity, Contact::new(normal_impulse));
                            data.contact_remove.insert(entity, Remove::new(&data.time, 0.01));
                            // add contact to breakable if metal or rubber
                            if data.rubber.contains(entity) || data.metal.contains(entity) {
                                data.contact.insert(interaction.with, Contact::new(normal_impulse));
                                data.contact_remove.insert(interaction.with, Remove::new(&data.time, 0.01));
                            }
                        }
                    }
                }
            }
        }
    }
}
