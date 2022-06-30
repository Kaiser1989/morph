#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;
use shrev::ReaderId;

use crate::game::resource::{Events};
use crate::game::state::level::{LevelEvent, SystemPhase};
use crate::game::state::level::component::{Role};


//////////////////////////////////////////////////
// Definition

pub struct RoleSystem {
    reader: Option<ReaderId<LevelEvent>>,
}

#[derive(SystemData)]
pub struct RoleSystemData<'a> {
    roles: WriteStorage<'a, Role>,
    events: Write<'a, Events<LevelEvent>>,
    phase: Read<'a, SystemPhase>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for RoleSystem { 

    type SystemData = RoleSystemData<'a>;

    fn run(&mut self, mut data : Self::SystemData) {

        // check events
        if *data.phase == SystemPhase::Events {
            for event in data.events.read_opt(&mut self.reader).into_iter() {
                match event {

                    // object actions
                    LevelEvent::ObjectBreak(entity, _, _) => {
                        data.roles.insert(entity, Role::Particle);
                    },

                    // ignore
                    _ => {}
                }
            }
        }
    }
}


//////////////////////////////////////////////////
// Implementation

impl RoleSystem { 
    
    pub fn new() -> RoleSystem { 
        RoleSystem{reader: None} 
    } 

    pub fn init(&mut self, world: &mut World) {
        self.reader = Some(world.get_mut::<Events<LevelEvent>>().unwrap().register());
    }
}