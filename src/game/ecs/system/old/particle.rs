//////////////////////////////////////////////////
// Using

use std::f32::consts::PI;

use nalgebra_glm::*;
use specs::prelude::*;
use shrev::ReaderId;
use rand::distributions::{Distribution, Uniform};

use crate::game::fx::*;
use crate::game::resource::{Events, Time, Plane, Animation, AnimationType};
use crate::game::state::level::{LevelEvent, SystemPhase};
use crate::game::state::level::component::{
    Emitter, Rendering, Lifetime, OpacityAnimation
};


//////////////////////////////////////////////////
// Definition

pub struct ParticleSystem {
    reader: Option<ReaderId<LevelEvent>>,
}

#[derive(SystemData)]
pub struct ParticleSystemData<'a> {
    emitters: WriteStorage<'a, Emitter>,
    entities: Entities<'a>,
    events: Write<'a, Events<LevelEvent>>,
    time: Read<'a, Time>,
    lazy: Read<'a, LazyUpdate>,
    phase: Read<'a, SystemPhase>,
}


//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for ParticleSystem { 

    type SystemData = ParticleSystemData<'a>;

    fn run(&mut self, mut data : Self::SystemData) {

        // check events
        if *data.phase == SystemPhase::Events {
            for event in data.events.read_opt(&mut self.reader).into_iter() {
                match event {
                    // ignore
                    _ => {}
                }
            };
        }

        // update
        if *data.phase == SystemPhase::Update {
            // update all particle emitters
            for emitter in (&mut data.emitters).join() {
                emitter.time -= data.time.0;
                if emitter.time <= 0.0 {
                    emitter.time = emitter.frequency;

                    // init random generator
                    let mut rng = rand::thread_rng();
                    let rotation_distribution = Uniform::new_inclusive(-PI, PI);

                    // create particles
                    for _ in 0 .. emitter.count {
                        // adjust physic
                        let mut physic_info = emitter.physic.clone();
                        physic_info.rotation = rotation_distribution.sample(&mut rng);
                        physic_info.linear_velocity = vec2(1.0, 1.0);

                        // create particle
                        data.lazy.create_entity(&data.entities)
                            .with(physic_info)
                            // TODO: KEEP ORIGINAL LAYER
                            .with(Rendering::new(TEX_GAME_MORPH, 0.0, 1.0, 1, Plane::View))
                            .with(Lifetime::new(1.5))
                            .with(OpacityAnimation(Animation::new(1.0, 0.0, 0.25, AnimationType::Once).delay(1.25)))
                            .build();
                    }
                }
            }
        }
    }
}


//////////////////////////////////////////////////
// Implementation

impl ParticleSystem { 
    
    pub fn new() -> ParticleSystem { 
        ParticleSystem{ reader: None } 
    } 

    pub fn init(&mut self, world: &mut World) {
        self.reader = Some(world.get_mut::<Events<LevelEvent>>().unwrap().register());
    }
}