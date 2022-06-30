#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use std::f32::consts::PI;

use nalgebra_glm::*;
use shrev::ReaderId;
use smallvec::*;
use specs::prelude::*;
use specs::WorldExt;

use crate::game::config::*;
use crate::game::fx::{GraphicsContext, TextureSrc};
use crate::game::resource::{Events, ResourceContext};

use crate::game::ecs::component::*;
use crate::game::ecs::event::*;
use crate::game::ecs::resource::*;
use crate::game::ecs::system::*;

use super::LevelEvent;

//////////////////////////////////////////////////
// Definition

#[derive(Default)]
struct Systems {
    input_morph: InputMorphSystem,
    input_camera: InputCameraSystem,
    physic_sync: PhysicSyncSystem,
    physic_force: PhysicForceSystem,
    physic_write: PhysicWriteSystem,
    physic_update: PhysicUpdateSystem,
    physic_follow: PhysicFollowSystem,
    physic_interaction: PhysicInteractionSystem,
    physic_read: PhysicReadSystem,
    story_interaction: StoryInteractionSystem,
    story_morph: StoryMorphSystem,
    story_morph_animation: StoryMorphAnimationSystem,
    story_object: StoryObjectSystem,
    story_object_animation: StoryObjectAnimationSystem,
    animation: AnimationSystem,
    lifetime: LifetimeSystem,
    output: OutputSystem,
    // role: RoleSystem,
    // animation: AnimationSystem,
    // lifetime: LifetimeSystem,
    // particle: ParticleSystem,
    render: RenderSystem,
}

pub struct Scene {
    reader: ReaderId<LevelEvent>,
    world: World,
    systems: Systems,
}

//////////////////////////////////////////////////
// Implementation

impl Scene {
    pub fn new(reader: ReaderId<LevelEvent>) -> Scene {
        let world = World::new();
        let systems = Systems::default();
        Scene { reader, world, systems }
    }

    pub fn init(&mut self, resource: &ResourceContext) {
        // init world
        self.world = World::new();

        // setup systems
        RunNow::setup(&mut self.systems.input_morph, &mut self.world);
        RunNow::setup(&mut self.systems.input_camera, &mut self.world);

        RunNow::setup(&mut self.systems.physic_sync, &mut self.world);
        RunNow::setup(&mut self.systems.physic_force, &mut self.world);
        RunNow::setup(&mut self.systems.physic_read, &mut self.world);
        RunNow::setup(&mut self.systems.physic_update, &mut self.world);
        RunNow::setup(&mut self.systems.physic_follow, &mut self.world);
        RunNow::setup(&mut self.systems.physic_interaction, &mut self.world);
        RunNow::setup(&mut self.systems.physic_write, &mut self.world);

        RunNow::setup(&mut self.systems.story_interaction, &mut self.world);
        RunNow::setup(&mut self.systems.story_morph, &mut self.world);
        RunNow::setup(&mut self.systems.story_morph_animation, &mut self.world);
        RunNow::setup(&mut self.systems.story_object, &mut self.world);
        RunNow::setup(&mut self.systems.story_object_animation, &mut self.world);

        RunNow::setup(&mut self.systems.animation, &mut self.world);
        RunNow::setup(&mut self.systems.lifetime, &mut self.world);
        RunNow::setup(&mut self.systems.output, &mut self.world);
        RunNow::setup(&mut self.systems.render, &mut self.world);

        // init world (entities, camera, ...)
        init_world(&mut self.world, resource);
    }

    pub fn update(&mut self, elapsed_time: f32, events: &mut Events<LevelEvent>) {
        // update events
        for event in events.read(&mut self.reader) {
            match event {
                // preview => running
                LevelEvent::Start => {
                    write_event(&mut self.world, EventSceneStart);
                }
                // running => finish
                LevelEvent::Success | LevelEvent::Failure => {
                    write_event(&mut self.world, EventSceneEnd);
                }

                // camera move event
                LevelEvent::MoveCamera(delta) => {
                    write_event(&mut self.world, EventCameraMove(delta));
                }

                // morph event
                LevelEvent::InputMorph(state) => {
                    write_event(&mut self.world, EventMorph(state));
                }

                _ => {}
            }
        }

        // update time
        if let Some(game_time) = self.world.get_mut::<GameTime>() {
            game_time.update(elapsed_time);
        }

        // update systems
        self.systems.input_morph.run_now(&self.world);
        self.systems.input_camera.run_now(&self.world);

        self.systems.physic_sync.run_now(&self.world);
        self.systems.physic_force.run_now(&self.world);
        self.systems.physic_read.run_now(&self.world);
        self.systems.physic_update.run_now(&self.world);
        self.systems.physic_follow.run_now(&self.world);
        self.systems.physic_interaction.run_now(&self.world);
        self.systems.physic_write.run_now(&self.world);

        self.systems.story_interaction.run_now(&self.world);
        self.systems.story_morph.run_now(&self.world);
        self.systems.story_morph_animation.run_now(&self.world);
        self.systems.story_object.run_now(&self.world);
        self.systems.story_object_animation.run_now(&self.world);

        self.systems.animation.run_now(&self.world);
        self.systems.lifetime.run_now(&self.world);

        self.systems.output.run_now(&self.world);

        // persist lazy updates, remove events
        self.world.maintain();

        // check for exit condition
        let output = self.world.read_resource::<Output>();
        match (output.exit, output.success) {
            (true, true) => events.write_delayed(LevelEvent::Success, output.delay),
            (true, false) => events.write_delayed(LevelEvent::Failure, output.delay),
            _ => (),
        }
    }

    pub fn draw(&mut self, graphics: &mut GraphicsContext) {
        self.systems.render.draw(&mut self.world, graphics);
    }
}

//////////////////////////////////////////////////
// World

fn init_world(world: &mut World, resource: &ResourceContext) {
    if let (Some(_package_info), Some(level_info)) = (resource.package_info(), resource.level_info()) {
        // get level infos
        let morph_info = &level_info.morph;
        let portal_info = &level_info.target;
        let morph_state = morph_info.state;

        // create level court
        world
            .create_entity()
            .with(Physic)
            .with(Position::new(vec2(0.0, 0.0)))
            .with(Rotation::new(0.0))
            .with(Role::Court.collision())
            .with(Role::Court.sensor())
            .with(Shape::Rect(level_info.dimension))
            .with(Court)
            .build();

        // create morph
        let morph_builder = world
            .create_entity()
            .with(Physic)
            .with(Position::new(morph_info.position))
            .with(Rotation::new(0.0))
            .with(Velocity::new(vec2(0.0, 0.0), 0.0))
            .with(morph_state.velocity_limit())
            .with(morph_state.velocity_damping())
            .with(morph_state.gravity())
            .with(morph_state.mass())
            .with(morph_state.collision())
            .with(morph_state.sensor())
            .with(morph_state.material())
            .with(morph_state.shape())
            .with(morph_state.texture())
            .with(Layer::new(Plane::View, morph_info.layer.max(1)));
        let morph_entity = match morph_state {
            MorphState::Bubble => morph_builder.with(Bubble),
            MorphState::Water => morph_builder.with(Water),
            MorphState::Rubber => morph_builder.with(Rubber),
            MorphState::Metal => morph_builder.with(Metal),
        }
        .build();

        // create portal
        let portal_entity = world
            .create_entity()
            .with(Physic)
            .with(Position::new(portal_info.position))
            .with(Rotation::new(0.0))
            .with(Role::Portal.collision())
            .with(Role::Portal.sensor())
            .with(Role::Portal.shape())
            .with(Role::Portal.texture())
            .with(Layer::new(Plane::View, portal_info.layer.max(morph_info.layer.max(1) + 1)))
            .with(Portal)
            .with(Animation::with_kind(smallvec![TextureSlot::new(0.0), TextureSlot::new(30.0)], 1.5, AnimationKind::Repeat))
            .with(Animation::with_kind(smallvec![Rotation::new(0.0), Rotation::new(PI * 2.0)], 10.0, AnimationKind::Repeat))
            .build();

        // create objects
        for object_info in level_info.objects.iter() {
            let mut builder = world
                .create_entity()
                .with(Physic)
                .with(Position::new(object_info.position))
                .with(Rotation::new(object_info.rotation))
                .with(object_info.role.collision())
                .with(object_info.role.sensor())
                .with(Shape::Rect(object_info.size));
            // adding texture?
            if object_info.texture >= 0 {
                let texture_info = object_info.texture_info.as_ref().unwrap();
                builder = builder.with(Texture::new(TextureSrc::Package(object_info.texture as usize)));
                builder = builder.with(Layer::new(texture_info.plane, texture_info.layer));
                // // adding animation?
                // let sub_tex_len = package_info.textures[object_info.texture as usize].len();
                // if sub_tex_len > 1 && texture_info.animation > 0.0 {
                //     builder = builder.with(TextureAnimation(Animation::new(0.0, sub_tex_len as f32, texture_info.animation, AnimationType::Repeat)));
                // }
            }
            // adding role
            match object_info.role {
                Role::Block => {
                    builder = builder.with(Block);
                }
                Role::Spikes => {
                    builder = builder.with(Spikes);
                }
                Role::Grid => {
                    builder = builder.with(Grid);
                }
                Role::Accelerator => {
                    let accelerator_info = object_info.accelerator.as_ref().unwrap();
                    let accelerator_direction: Vec2 = accelerator_info.direction.into();
                    builder = builder.with(Accelerator::new(accelerator_direction * accelerator_info.amplitude));
                    let morphs: Vec<usize> = accelerator_info.morph.iter().filter(|&(_, v)| *v).map(|(k, _)| k.sensor().group).collect();
                    builder = builder.with(Role::Accelerator.collision());
                    builder = builder.with(Sensor {
                        group: Role::Accelerator.sensor().group,
                        with: morphs,
                    });
                }
                Role::Breakable => {
                    let breakable_info = object_info.breakable.as_ref().unwrap();
                    builder = builder.with(Breakable::new(breakable_info.group));
                }
                _ => (),
            }
            builder.build();
        }

        // create camera
        let camera_entity = world
            .create_entity()
            .with(Physic)
            .with(Dynamic)
            .with(Position::new(level_info.morph.position))
            .with(Rotation::new(0.0))
            .with(Velocity::new(vec2(0.0, 0.0), 0.0))
            .with(VelocityLimit::new(15.0, 0.0))
            .with(VelocityDamping::new(CONFIG.level_camera_damping, 0.0))
            .with(Camera::new(CONFIG.level_camera_zoom, level_info.dimension))
            .build();

        // init resources
        world.insert(Physix::new());
        world.insert(GameTime::new(0.0, 0.0));
        world.insert(Actors::new(camera_entity, morph_entity, portal_entity));
        world.insert(Output::default());
    }
}
