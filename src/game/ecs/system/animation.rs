#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use nalgebra_glm::*;
use specs::prelude::*;
use specs::storage::GenericWriteStorage;

use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct AnimationSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    time: Read<'a, GameTime>,

    // write components
    rotation: WriteStorage<'a, Rotation>,
    rotation_anim: WriteStorage<'a, Animation<Rotation>>,
    rotation_anim_remove: WriteStorage<'a, Remove<Animation<Rotation>>>,
    shape: WriteStorage<'a, Shape>,
    shape_anim: WriteStorage<'a, Animation<Shape>>,
    shape_anim_remove: WriteStorage<'a, Remove<Animation<Shape>>>,
    texture_slot: WriteStorage<'a, TextureSlot>,
    texture_slot_anim: WriteStorage<'a, Animation<TextureSlot>>,
    texture_slot_anim_remove: WriteStorage<'a, Remove<Animation<TextureSlot>>>,
    opacity: WriteStorage<'a, Opacity>,
    opacity_anim: WriteStorage<'a, Animation<Opacity>>,
    opacity_anim_remove: WriteStorage<'a, Remove<Animation<Opacity>>>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for AnimationSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // rotate animation
        update_animation(
            &data.entities,
            &data.time,
            &mut data.rotation,
            &mut data.rotation_anim,
            &mut data.rotation_anim_remove,
        );

        // shape animation
        update_animation(
            &data.entities,
            &data.time,
            &mut data.shape,
            &mut data.shape_anim,
            &mut data.shape_anim_remove,
        );

        // texture slot animation
        update_animation(
            &data.entities,
            &data.time,
            &mut data.texture_slot,
            &mut data.texture_slot_anim,
            &mut data.texture_slot_anim_remove,
        );

        // opacity animation
        update_animation(
            &data.entities,
            &data.time,
            &mut data.opacity,
            &mut data.opacity_anim,
            &mut data.opacity_anim_remove,
        );
    }
}

//////////////////////////////////////////////////
// Helper

#[inline]
fn update_animation<C>(
    entities: &Entities,
    time: &Read<GameTime>,
    comp_storage: &mut WriteStorage<C>,
    anim_storage: &mut WriteStorage<Animation<C>>,
    remove_storage: &mut WriteStorage<Remove<Animation<C>>>,
) where
    C: Animatable + Component + Send + Sync,
{
    for (entity, mut anim) in (entities, anim_storage).join() {
        let index =
            clamp_scalar(anim.current / anim.duration, 0.0, 1.0) * (anim.frames.len() - 1) as f32;
        let lower = anim.frames.get(index.floor() as usize).unwrap();
        let upper = anim.frames.get(index.ceil() as usize).unwrap();
        let comp = comp_storage.get_mut_or_default(entity).unwrap();
        *comp = lower.interpolate(upper, index.fract());
        anim.current += time.frame_time;
        if anim.current >= anim.duration {
            match anim.kind {
                AnimationKind::Single => {
                    remove_storage.insert(entity, Remove::new(time, 0.0));
                }
                AnimationKind::Repeat => {
                    anim.current -= anim.duration;
                }
            }
        }
    }
}
