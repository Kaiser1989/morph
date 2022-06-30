#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct LifetimeSystem;

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    lazy: Read<'a, LazyUpdate>,
    time: Read<'a, GameTime>,

    // write components
    lifetime: WriteStorage<'a, Lifetime>,
    shape_anim: WriteStorage<'a, Animation<Shape>>,
    shape_anim_remove: WriteStorage<'a, Remove<Animation<Shape>>>,
    contact: WriteStorage<'a, Contact>,
    contact_remove: WriteStorage<'a, Remove<Contact>>,
    squeeze: WriteStorage<'a, Squeeze>,
    squeeze_remove: WriteStorage<'a, Remove<Squeeze>>,
    surprise: WriteStorage<'a, Surprise>,
    surprise_remove: WriteStorage<'a, Remove<Surprise>>,
    blink: WriteStorage<'a, Blink>,
    blink_remove: WriteStorage<'a, Remove<Blink>>,
    opacity_anim: WriteStorage<'a, Animation<Opacity>>,
    opacity_anim_insert: WriteStorage<'a, Insert<Animation<Opacity>>>,
    opacity_anim_remove: WriteStorage<'a, Remove<Animation<Opacity>>>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for LifetimeSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // update entities' lifetimes
        for (entity, lifetime) in (&data.entities, &mut data.lifetime).join() {
            if data.time.all_time >= lifetime.0 {
                data.entities.delete(entity);
            }
        }

        // update comp inserts
        update_insert(
            &data.entities,
            &data.lazy,
            &data.time,
            &mut data.opacity_anim,
            &mut data.opacity_anim_insert,
        );

        // update comp removes
        update_remove(
            &data.entities,
            &data.lazy,
            &data.time,
            &mut data.shape_anim,
            &mut data.shape_anim_remove,
        );
        update_remove(
            &data.entities,
            &data.lazy,
            &data.time,
            &mut data.contact,
            &mut data.contact_remove,
        );
        update_remove(
            &data.entities,
            &data.lazy,
            &data.time,
            &mut data.squeeze,
            &mut data.squeeze_remove,
        );
        update_remove(
            &data.entities,
            &data.lazy,
            &data.time,
            &mut data.surprise,
            &mut data.surprise_remove,
        );
        update_remove(
            &data.entities,
            &data.lazy,
            &data.time,
            &mut data.blink,
            &mut data.blink_remove,
        );
        update_remove(
            &data.entities,
            &data.lazy,
            &data.time,
            &mut data.opacity_anim,
            &mut data.opacity_anim_remove,
        );
    }
}

//////////////////////////////////////////////////
// Helper

#[inline]
fn update_insert<C>(
    entities: &Entities,
    lazy: &Read<LazyUpdate>,
    time: &Read<GameTime>,
    comp_storage: &mut WriteStorage<C>,
    insert_storage: &mut WriteStorage<Insert<C>>,
) where
    C: Component + Send + Sync,
{
    for (entity, insert) in (entities, insert_storage).join() {
        if time.all_time >= insert.lifetime {
            comp_storage.insert(entity, insert.component.take().unwrap());
            lazy.remove::<Insert<C>>(entity);
        }
    }
}

#[inline]
fn update_remove<C>(
    entities: &Entities,
    lazy: &Read<LazyUpdate>,
    time: &Read<GameTime>,
    comp_storage: &mut WriteStorage<C>,
    remove_storage: &mut WriteStorage<Remove<C>>,
) where
    C: Component + Send + Sync,
{
    for (entity, remove) in (entities, remove_storage).join() {
        if time.all_time >= remove.lifetime {
            comp_storage.remove(entity);
            lazy.remove::<Remove<C>>(entity);
        }
    }
}
