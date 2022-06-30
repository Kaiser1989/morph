#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use nalgebra_glm::*;
use smallvec::*;
use specs::prelude::*;

use crate::game::ecs::component::*;
use crate::game::ecs::resource::*;
use crate::game::fx::*;
use crate::game::resource::ComponentTracker;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
pub struct StoryMorphAnimationSystem {
    bubble_tracker: ComponentTracker<Bubble>,
    water_tracker: ComponentTracker<Water>,
    rubber_tracker: ComponentTracker<Rubber>,
    metal_tracker: ComponentTracker<Metal>,
    finish_tracker: ComponentTracker<Finish>,
    burst_tracker: ComponentTracker<Burst>,
}

#[derive(SystemData)]
pub struct Data<'a> {
    // resources
    entities: Entities<'a>,
    time: Read<'a, GameTime>,

    // write components
    physic: WriteStorage<'a, Physic>,
    position: WriteStorage<'a, Position>,
    rotation_anim: WriteStorage<'a, Animation<Rotation>>,
    shape: WriteStorage<'a, Shape>,
    shape_anim: WriteStorage<'a, Animation<Shape>>,
    follow: WriteStorage<'a, Follow>,
    lifetime: WriteStorage<'a, Lifetime>,
    texture: WriteStorage<'a, Texture>,
    texture_slot: WriteStorage<'a, TextureSlot>,
    texture_slot_anim: WriteStorage<'a, Animation<TextureSlot>>,
    layer: WriteStorage<'a, Layer>,
    squeeze: WriteStorage<'a, Squeeze>,
    squeeze_remove: WriteStorage<'a, Remove<Squeeze>>,
    surprise: WriteStorage<'a, Surprise>,
    surprise_remove: WriteStorage<'a, Remove<Surprise>>,
    blink: WriteStorage<'a, Blink>,
    blink_remove: WriteStorage<'a, Remove<Blink>>,

    // read components
    velocity: ReadStorage<'a, Velocity>,
    acceleration: ReadStorage<'a, Acceleration>,
    slow: ReadStorage<'a, Slow>,
    contact: ReadStorage<'a, Contact>,
    finish: ReadStorage<'a, Finish>,
    burst: ReadStorage<'a, Burst>,
    bubble: ReadStorage<'a, Bubble>,
    water: ReadStorage<'a, Water>,
    rubber: ReadStorage<'a, Rubber>,
    metal: ReadStorage<'a, Metal>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for StoryMorphAnimationSystem {
    type SystemData = Data<'a>;

    fn setup(&mut self, res: &mut World) {
        Self::SystemData::setup(res);
        self.bubble_tracker.setup(res);
        self.water_tracker.setup(res);
        self.rubber_tracker.setup(res);
        self.metal_tracker.setup(res);
        self.finish_tracker.setup(res);
        self.burst_tracker.setup(res);
    }

    fn run(&mut self, mut data: Self::SystemData) {
        // update trackers
        self.bubble_tracker.update(&data.bubble);
        self.water_tracker.update(&data.water);
        self.rubber_tracker.update(&data.rubber);
        self.metal_tracker.update(&data.metal);
        self.finish_tracker.update(&data.finish);
        self.burst_tracker.update(&data.burst);

        // calculate masks
        let morph_mask = data.bubble.mask() | data.water.mask() | data.rubber.mask() | data.metal.mask();

        // morph change animation
        for (morph_entity, _) in (
            &data.entities,
            self.bubble_tracker.inserted() | self.water_tracker.inserted() | self.rubber_tracker.inserted() | self.metal_tracker.inserted(),
        )
            .join()
        {
            // get position form moprh
            let morph_position = *data.position.get(morph_entity).unwrap();
            let morph_shape = *data.shape.get(morph_entity).unwrap();
            let morph_layer = *data.layer.get(morph_entity).unwrap();

            // create animation entity
            let entity = data.entities.create();
            data.physic.insert(entity, Physic);
            data.follow.insert(entity, Follow(morph_entity));
            data.position.insert(entity, morph_position);
            data.shape.insert(entity, morph_shape * 1.5);
            data.texture.insert(entity, Texture::new(TEX_GAME_MORPH));
            data.texture_slot_anim.insert(entity, Animation::new(smallvec![TextureSlot::new(0.0), TextureSlot::new(15.0)], 0.25));
            data.layer.insert(entity, Layer::new(Plane::View, morph_layer.rank - 1));
            data.lifetime.insert(entity, Lifetime::new(&data.time, 0.25));
        }

        // morph burst animation
        for (entity, _) in (&data.entities, self.burst_tracker.inserted()).join() {
            // bubble
            if data.bubble.contains(entity) {
                data.texture.insert(entity, Texture::new(TEX_GAME_BUBBLE_BURST));
                data.texture_slot_anim.insert(entity, Animation::new(smallvec![TextureSlot::new(0.0), TextureSlot::new(8.0)], 0.25));
            }

            // rubber
            if data.rubber.contains(entity) {
                data.texture.insert(entity, Texture::new(TEX_GAME_RUBBER_BURST));
                data.texture_slot_anim.insert(entity, Animation::new(smallvec![TextureSlot::new(0.0), TextureSlot::new(5.0)], 0.30));
                data.rotation_anim.insert(
                    entity,
                    Animation::with_kind(smallvec![Rotation::new(0.0), Rotation::new(0.8), Rotation::new(-0.8), Rotation::new(0.0)], 1.2, AnimationKind::Repeat),
                );
            }
        }

        // morph finish animation
        for (entity, _) in (&data.entities, &morph_mask & self.finish_tracker.inserted()).join() {
            let current_shape = *data.shape.get(entity).unwrap();
            data.shape_anim.insert(entity, Animation::new(smallvec![current_shape, Shape::Ball(0.0)], 2.0));
        }

        // morph face animations
        for (entity, _, _, _) in (&data.entities, &morph_mask, !&data.burst, !&data.finish).join() {
            // get morph velocity
            let morph_velocity = data.velocity.get(entity).unwrap();

            // surprise animation
            // (velocity linear)
            if length2(&morph_velocity.0) > 24.9 {
                data.surprise.insert(entity, Surprise);
                data.surprise_remove.insert(entity, Remove::new(&data.time, 0.01));
            }
            // (velocity angular)
            if morph_velocity.1 > 2.0 {
                data.surprise.insert(entity, Surprise);
                data.surprise_remove.insert(entity, Remove::new(&data.time, 0.01));
            }
            // (acceleration)
            if data.acceleration.contains(entity) {
                data.surprise.insert(entity, Surprise);
                data.surprise_remove.insert(entity, Remove::new(&data.time, 0.01));
            }

            // squeeze animation
            // (finish)
            if data.finish.contains(entity) {
                data.squeeze.insert(entity, Squeeze);
            }
            // (bounce)
            if length2(&morph_velocity.0) > 4.0 && data.contact.contains(entity) {
                data.squeeze.insert(entity, Squeeze);
                data.squeeze_remove.insert(entity, Remove::new(&data.time, 0.3));
            }
            // (slow)
            if data.slow.contains(entity) {
                data.squeeze.insert(entity, Squeeze);
                data.squeeze_remove.insert(entity, Remove::new(&data.time, 0.01));
            }

            // blink animation
            if (data.time.all_time * 10.0) as i32 % 30 == 0 {
                data.blink.insert(entity, Blink);
                data.blink_remove.insert(entity, Remove::new(&data.time, 0.1));
            } else {
                data.blink.remove(entity);
            }

            // update morph face
            if data.squeeze.contains(entity) {
                data.texture_slot.insert(entity, TextureSlot::new(SLOT_MORPH_SQUEEZE));
            } else if data.surprise.contains(entity) {
                data.texture_slot.insert(entity, TextureSlot::new(SLOT_MORPH_SURPRISE));
            } else if data.blink.contains(entity) {
                data.texture_slot.insert(entity, TextureSlot::new(SLOT_MORPH_BLINK));
            } else {
                data.texture_slot.insert(entity, TextureSlot::new(SLOT_MORPH_NORMAL));
            }
        }
    }
}
