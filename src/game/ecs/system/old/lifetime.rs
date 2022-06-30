#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;
use shrev::ReaderId;

use crate::game::resource::{Events, Time};
use crate::game::state::level::{LevelEvent, SystemPhase};
use crate::game::state::level::component::{
    Lifetime, Deleted
};


//////////////////////////////////////////////////
// Definition

pub struct LifetimeSystem {
    reader: Option<ReaderId<LevelEvent>>,
}

#[derive(SystemData)]
pub struct LifetimeSystemData<'a> {
    lifetimes: WriteStorage<'a, Lifetime>,
    entities: Entities<'a>,
    events: Write<'a, Events<LevelEvent>>,
    time: Read<'a, Time>,
    lazy: Read<'a, LazyUpdate>,
    phase: Read<'a, SystemPhase>,
}


//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for LifetimeSystem { 

    type SystemData = LifetimeSystemData<'a>;

    fn run(&mut self, mut data : Self::SystemData) {

        // // check events
        // if *data.phase == SystemPhase::Events {
        //     for event in data.events.read_opt(&mut self.reader).into_iter() {
        //         match event {

        //             // morph actions
        //             LevelEvent::MorphBubbleBurst(entity) => {
        //                 data.lifetimes.insert(entity, Lifetime::new(1.0));
        //             },
        //             LevelEvent::MorphRubberBurst(entity) => {
        //                 data.lifetimes.insert(entity, Lifetime::new(1.5));
        //             },

        //             // object actions
        //             LevelEvent::ObjectBreak(entity, _, _) => {
        //                 data.lifetimes.insert(entity, Lifetime::new(2.0));
        //             },

        //             // Create entities
        //             LevelEvent::CreateMorphEffect(entity, _) => {
        //                 data.lifetimes.insert(entity, Lifetime::new(0.25));
        //             },

        //             // ignore
        //             _ => {}
        //         }
        //     };
        // }

        // // update
        // if *data.phase == SystemPhase::Update {
        //     // update all lifetimes
        //     for (lifetime, entity) in (&mut data.lifetimes, &data.entities).join() {
        //         lifetime.time -= data.time.0;
        //         if lifetime.time <= 0.0 {
        //             data.lazy.insert(entity, Delete::new());                
        //         }
        //     }
        // }

        for (lifetime, entity) in (&mut data.lifetimes, &data.entities).join() {
            lifetime.value -= data.time.0;
        }
    }
}


//////////////////////////////////////////////////
// Implementation

impl LifetimeSystem { 
    
    pub fn new() -> LifetimeSystem { 
        LifetimeSystem{ reader: None } 
    } 

    pub fn init(&mut self, world: &mut World) {
        self.reader = Some(world.get_mut::<Events<LevelEvent>>().unwrap().register());
    }
}