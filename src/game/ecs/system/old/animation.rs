#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;
use shrev::ReaderId;
use nalgebra_glm::*;

use crate::game::fx::*;
use crate::game::config::*;
use crate::game::resource::{Physix, Events, Time, Animation, AnimationType};
use crate::game::state::level::{LevelEvent, SystemPhase};
use crate::game::state::level::component::{
    PhysicHandle, TextureAnimation, OpacityAnimation, TranslateAnimation, ScaleAnimation, RotateAnimation, Rendering
};


//////////////////////////////////////////////////
// Definition

pub struct AnimationSystem {
    reader: Option<ReaderId<LevelEvent>>,
}

#[derive(SystemData)]
pub struct AnimationSystemData<'a> {
    texture_animations: WriteStorage<'a, TextureAnimation>,
    translate_animations: WriteStorage<'a, TranslateAnimation>,
    opacity_animations: WriteStorage<'a, OpacityAnimation>,
    scale_animations: WriteStorage<'a, ScaleAnimation>,
    rotate_animations: WriteStorage<'a, RotateAnimation>,
    physic_handles: ReadStorage<'a, PhysicHandle>,
    renderings: WriteStorage<'a, Rendering>,
    entities: Entities<'a>,
    physix: Write<'a, Physix>,
    events: Write<'a, Events<LevelEvent>>,
    lazy: Read<'a, LazyUpdate>,
    time: Read<'a, Time>,
    phase: Read<'a, SystemPhase>,
}


//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for AnimationSystem { 

    type SystemData = AnimationSystemData<'a>;

    fn run(&mut self, mut data : Self::SystemData) {
        
        // check events
        if *data.phase == SystemPhase::Events {
            for event in data.events.read_opt(&mut self.reader).into_iter() {
                match event {

                    // morph actions
                    LevelEvent::MorphTarget(entity, _) => {
                        data.scale_animations.insert(entity, ScaleAnimation(Animation::new(vec2(CONFIG.level_morph_size, CONFIG.level_morph_size), vec2(0.0, 0.0), 2.0, AnimationType::Once)));
                    },
                    LevelEvent::MorphBubbleBurst(entity) => {
                        data.texture_animations.insert(entity, TextureAnimation(Animation::new(0.0, 8.0, 0.25, AnimationType::Once).priority(3))); 
                    },
                    LevelEvent::MorphRubberBurst(entity) => {
                        data.texture_animations.insert(entity, TextureAnimation(Animation::new(0.0, 5.0, 0.3, AnimationType::Once).priority(3)));
                        data.rotate_animations.insert(entity, RotateAnimation(Animation::new(-0.8, 0.8, 0.6, AnimationType::Reverse).current(0.3)));
                    },
                    LevelEvent::MorphSqueeze(entity, priority) => {
                        if priority >= data.texture_animations.get(entity).map(|anim| anim.0.priority).unwrap_or(0) {
                            data.texture_animations.insert(entity, TextureAnimation(Animation::new(SLOT_MORPH_SQUEEZE, SLOT_MORPH_SQUEEZE, 0.2, AnimationType::Once).after(SLOT_MORPH_NORMAL).priority(priority)));
                        }
                    },
                    LevelEvent::MorphSurprise(entity, priority) => {
                        if priority >= data.texture_animations.get(entity).map(|anim| anim.0.priority).unwrap_or(0) {
                            data.texture_animations.insert(entity, TextureAnimation(Animation::new(SLOT_MORPH_SURPRISE, SLOT_MORPH_SURPRISE, 0.1, AnimationType::Once).after(SLOT_MORPH_NORMAL).priority(priority)));
                        }
                    },
                    LevelEvent::MorphBlink(entity, priority) => {
                        if priority >= data.texture_animations.get(entity).map(|anim| anim.0.priority).unwrap_or(0) {
                            data.texture_animations.insert(entity, TextureAnimation(Animation::new(SLOT_MORPH_BLINK, SLOT_MORPH_BLINK, 0.1, AnimationType::Once).after(SLOT_MORPH_NORMAL).priority(priority)));
                        }
                    },

                    // object actions
                    LevelEvent::ObjectBreak(entity, ..) => {
                        data.opacity_animations.insert(entity, OpacityAnimation(Animation::new(1.0, 0.0, 0.5, AnimationType::Once).delay(1.5)));
                    },

                    // Create entities
                    LevelEvent::CreateMorphEffect(entity, _) => {
                        data.texture_animations.insert(entity, TextureAnimation(Animation::new(0.0, 15.0, 0.25, AnimationType::Once)));
                    },

                    // ignore
                    _ => {}
                }
            };
        }

        // update
        if *data.phase == SystemPhase::Update {
            // get time from resource
            let elapsed_time = data.time.0;

            // texture animations
            for (entity, animation, rendering) in (&data.entities, &mut data.texture_animations, &mut data.renderings).join() {
                // update animation
                animation.0.update(elapsed_time);
                // check animation end
                if animation.0.finish { 
                    rendering.texture_slot = animation.0.after;
                    data.lazy.remove::<TextureAnimation>(entity); 
                } else {
                    rendering.texture_slot = animation.0.value(|&s, &e, t| lerp_scalar(s, e, t));
                }
            }

            // opacity animations
            for (entity, animation, rendering) in (&data.entities, &mut data.opacity_animations, &mut data.renderings).join() {
                // update animation
                animation.0.update(elapsed_time);
                // check animation end
                if animation.0.finish { 
                    rendering.opacity = animation.0.after;
                    data.lazy.remove::<OpacityAnimation>(entity); 
                } else {
                    rendering.opacity = animation.0.value(|&s, &e, t| lerp_scalar(s, e, t));
                }
            }

            // translate animations
            for (entity, animation, physic_handle) in (&data.entities, &mut data.translate_animations, &data.physic_handles).join() {
                // update animation
                animation.0.update(elapsed_time);
                // check animation end
                if animation.0.finish { 
                    data.physix.change_position(physic_handle, animation.0.after);
                    data.lazy.remove::<TranslateAnimation>(entity); 
                } else {
                    data.physix.change_position(physic_handle, animation.0.value(|s, e, t| lerp(s, e, t)));
                }
            }

            // scale animations
            for (entity, animation, physic_handle) in (&data.entities, &mut data.scale_animations, &data.physic_handles).join() {
                // update animation
                animation.0.update(elapsed_time);
                // check animation end
                if animation.0.finish { 
                    data.physix.change_size(physic_handle, animation.0.after);
                    data.lazy.remove::<ScaleAnimation>(entity); 
                } else {
                    data.physix.change_size(physic_handle, animation.0.value(|s, e, t| lerp(s, e, t)));
                }
            }

            // rotate animations
            for (entity, animation, physic_handle) in (&data.entities, &mut data.rotate_animations, &data.physic_handles).join() {
                // update animation
                animation.0.update(elapsed_time);
                // check animation end
                if animation.0.finish { 
                    data.physix.change_rotation(physic_handle, animation.0.after);
                    data.lazy.remove::<ScaleAnimation>(entity); 
                } else {
                    data.physix.change_rotation(physic_handle, animation.0.value(|&s, &e, t| lerp_scalar(s, e, t)));
                }
            }
        }
    }
}


//////////////////////////////////////////////////
// Implementation

impl AnimationSystem { 
    
    pub fn new() -> AnimationSystem { 
        AnimationSystem{reader: None} 
    } 

    pub fn init(&mut self, world: &mut World) {
        self.reader = Some(world.get_mut::<Events<LevelEvent>>().unwrap().register());
    }
}