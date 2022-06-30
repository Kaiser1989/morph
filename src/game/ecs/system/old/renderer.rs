#![allow(unused_must_use)]
//////////////////////////////////////////////////
// Using

use std::cmp::Ordering;

use enum_map::enum_map;
use game_gl::gl;
use itertools::Itertools;
use nalgebra_glm::*;
use shrev::ReaderId;
use specs::prelude::*;

use crate::game::fx::{GraphicsContext, Instance, TextureSrc};
use crate::game::resource::Events;

use crate::game::ecs::component::*;
use crate::game::ecs::event::*;
use crate::game::ecs::resource::*;

//////////////////////////////////////////////////
// Definition

pub struct RenderSystem {
    reader: Option<ReaderId<SceneEvent>>,
}
impl Default for RenderSystem {
    fn default() -> Self {
        RenderSystem::new()
    }
}

#[derive(SystemData)]
pub struct RenderSystemData<'a> {
    entities: Entities<'a>,
    position: WriteStorage<'a, Position>,
    shape: ReadStorage<'a, Shape>,
    texture: ReadStorage<'a, Texture>,
    texture_slot: ReadStorage<'a, TextureSlot>,
    layer: ReadStorage<'a, Layer>,
    _color: ReadStorage<'a, Color>,
    opacity: ReadStorage<'a, Opacity>,
    camera: ReadStorage<'a, Camera>,
    // renderings: WriteStorage<'a, Rendering>,
    // events: Read<'a, Events<LevelEvent>>,
    // phase: Read<'a, SystemPhase>,
}

//////////////////////////////////////////////////
// Trait Implementation

impl<'a> System<'a> for RenderSystem {
    type SystemData = RenderSystemData<'a>;

    // rendering
    fn run(&mut self, mut data: Self::SystemData) {

        // // check events
        // if *data.phase == SystemPhase::Events {
        //     for event in data.events.read_opt(&mut self.reader).into_iter() {
        //         match event {

        //             // morph actions
        //             LevelEvent::MorphChange(entity, state, _) => {
        //                 let render = data.renderings.get_mut(entity).unwrap();
        //                 render.texture = state.texture();
        //             },
        //             LevelEvent::MorphBubbleBurst(entity) => {
        //                 data.renderings.insert(entity, Rendering::new(TEX_GAME_BUBBLE_BURST, 0.0, 1.0, 1, Plane::View));
        //             },
        //             LevelEvent::MorphRubberBurst(entity) => {
        //                 data.renderings.insert(entity, Rendering::new(TEX_GAME_RUBBER_BURST, 0.0, 1.0, 1, Plane::View));
        //             },

        //             // Create Entity
        //             LevelEvent::CreateMorphEffect(entity, _) => {
        //                 data.renderings.insert(entity, Rendering::new(TEX_GAME_MORPH, 0.0, 1.0, 1, Plane::View));
        //             },

        //             // ignore
        //             _ => {}
        //         }
        //     };
        // }
    }
}

//////////////////////////////////////////////////
// Implementation

impl RenderSystem {
    pub fn new() -> RenderSystem {
        RenderSystem { reader: None }
    }

    pub fn init(&mut self, world: &mut World) {
        self.reader = Some(world.get_mut::<Events<SceneEvent>>().unwrap().register());
    }

    pub fn draw(&self, world: &mut World, graphics: &mut GraphicsContext) {
        // get world data
        let mut data: RenderSystemData = world.system_data();

        // create instances; sort by plane -> layer -> texture
        let mut instances: Vec<(Plane, TextureSrc, Instance)> = (
            &data.entities,
            &data.position,
            &data.shape,
            &data.texture,
            (&data.texture_slot).maybe(),
            (&data.opacity).maybe(),
            &data.layer,
        )
            .join()
            .map(|(_, position, shape, texture, texture_slot, opacity, layer)| {
                let instance = Instance {
                    translate: position.0,
                    rotate: position.1,
                    scale: shape.size(),
                    layer: -(layer.plane.layer() + (layer.rank as f32) / 10.0), // inverse layer
                    tex_slot: texture_slot.unwrap_or_default().0,
                    opacity: opacity.unwrap_or_default().0,
                    // TODO: Add Color
                };
                (layer.plane, texture.0, instance)
            })
            .collect();
        instances.sort_unstable_by(|(p0, t0, i0), (p1, t1, i1)| match p0.cmp(p1) {
            Ordering::Equal => match i0.layer.partial_cmp(&i1.layer).unwrap() {
                Ordering::Equal => t0.cmp(t1),
                x => x,
            },
            x => x,
        });

        // create camera view
        let view_proj = if let Some((position, camera)) =
            (&mut data.position, &data.camera).join().next()
        {
            // calculate aspect ratio
            let resolution = graphics.resolution();
            let aspect_ratio = resolution.x / resolution.y;
            let aspect_vec = if aspect_ratio > 1.0 {
                vec2(aspect_ratio, 1.0)
            } else {
                vec2(1.0, 1.0 / aspect_ratio)
            };

            // calculate zoom
            let max_zoom = comp_min(&vec2(
                camera.max_dimension.x / aspect_vec.x,
                camera.max_dimension.y / aspect_vec.y,
            ));
            let zoom = camera.zoom.min(max_zoom);

            // calculate position
            let dimension = aspect_vec * zoom;
            let cam_space = abs(&(camera.max_dimension - dimension));
            position.0 = min2(&max2(&position.0, &-cam_space), &cam_space);

            // calc ortho
            let proj = ortho_rh(
                -dimension.x,
                dimension.x,
                -dimension.y,
                dimension.y,
                0.1,
                10.0,
            );

            // calc view
            let eye = vec3(position.0.x, position.0.y, 0.0);
            enum_map! {
                Plane::View => proj * look_at_rh(&eye, &(eye - Vec3::z()), &Vec3::y()),
                Plane::Far => proj * look_at_rh(&(eye * 0.8), &(eye * 0.8 - Vec3::z()), &Vec3::y()),
                Plane::Mid => proj * look_at_rh(&(eye * 0.9), &(eye * 0.9 - Vec3::z()), &Vec3::y()),
                Plane::Near => proj * look_at_rh(&(eye * 1.1), &(eye * 1.1 - Vec3::z()), &Vec3::y()),
            }
        } else {
            enum_map! {
                Plane::View => Mat4::identity(),
                Plane::Far => Mat4::identity(),
                Plane::Mid => Mat4::identity(),
                Plane::Near => Mat4::identity(),
            }
        };

        // bind shader
        graphics.quad_shader.bind();
        graphics.quad_shader.link_texture(1, "t_textures");
        graphics.quad_shader.link_uniform(1, "Locals");
        // bind vao
        graphics.quad_vao.bind();
        // bind index buffer
        graphics.quad_ibo.bind();

        // render by plane
        for (plane, v) in &instances.into_iter().group_by(|(p, _, _)| *p) {
            // bind uniforms
            graphics.quad_ubo.update(&view_proj[plane]);
            graphics.quad_ubo.bind(1);

            // render by texture
            for (texture, v) in &v.into_iter().group_by(|(_, t, _)| *t) {
                // bind instances
                let instances: Vec<Instance> = v.into_iter().map(|(_, _, i)| i).collect();
                graphics.quad_inbo.update(&instances);
                // bind textures
                graphics.find_texture(texture).bind(1);

                // draw
                graphics.quad_shader.draw_elements_instanced(
                    gl::TRIANGLE_STRIP,
                    graphics.quad_ibo.count(),
                    instances.len(),
                );

                // unbind textures
                graphics.find_texture(texture).unbind();
            }

            // unbind uniforms
            graphics.quad_ubo.unbind();
        }

        // unbind all
        graphics.quad_ibo.unbind();
        // unbind vao
        graphics.quad_vao.unbind();
        // unbind shader
        graphics.quad_shader.unbind();
    }
}
