//////////////////////////////////////////////////
// Using

use std::mem::size_of;

use game_gl::opengl::*;
use game_gl::prelude::*;
use image::imageops::{resize, FilterType};
use image::{GrayImage, Luma, RgbaImage};
use nalgebra_glm::*;
use rusttype::{point, Font, Scale};

use crate::game::config::*;
use crate::game::fx::textures::*;
use crate::game::resource::*;

//////////////////////////////////////////////////
// Data

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct Vertex {
    pub pos: Vec2,
    pub tex_coord: Vec2,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct Instance {
    pub translate: Vec2,
    pub scale: Vec2,
    pub layer: f32,
    pub rotate: f32,
    pub tex_slot: f32,
    pub opacity: f32,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct GuiInstance {
    pub translate: Vec2,
    pub scale: Vec2,
    pub color: Vec4,
    pub layer: f32,
    pub tex_slot: f32,
    pub slice: f32,
    pub radius: f32,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct GlyphInstance {
    pub translate: Vec2,
    pub scale: Vec2,
    pub color: Vec4,
    pub layer: f32,
    pub unicode: f32,
    pub width: f32,
}

const MAX_INSTANCES: usize = 2048;
const MAX_GUI_INSTANCES: usize = 2048;
const MAX_GLYPH_INSTANCES: usize = 4096;

//////////////////////////////////////////////////
// Definition

#[derive(Debug, Default)]
pub struct GraphicsContext {
    // gl
    gl: Option<Gl>,

    // vertex array objects
    pub quad_vao: GlVertexArrayObject,
    pub gui_vao: GlVertexArrayObject,
    pub glyph_vao: GlVertexArrayObject,

    // const buffers
    pub quad_vbo: GlVertexBuffer<Vertex>,
    pub quad_ibo: GlIndexBuffer,

    // instances
    pub quad_inbo: GlVertexBuffer<Instance>,
    pub gui_inbo: GlVertexBuffer<GuiInstance>,
    pub glyph_inbo: GlVertexBuffer<GlyphInstance>,

    // uniforms
    pub quad_ubo: GlUniformBuffer<Mat4>,
    pub gui_ubo: GlUniformBuffer<Mat4>,
    pub glyph_ubo: GlUniformBuffer<Mat4>,

    // shader
    pub quad_shader: GlShader,
    pub gui_shader: GlShader,
    pub glyph_shader: GlShader,

    // textures
    pub glyph_texture: GlTexture,
    gui_textures: Vec<GlTexture>,
    game_textures: Vec<GlTexture>,
    package_textures: Vec<GlTexture>,

    // current screen resolution
    resolution: Vec2,
}

//////////////////////////////////////////////////
// Implementation

impl GraphicsContext {
    //////////////////////////////////////////////////
    // Device functions

    pub fn create(&mut self, ctx: &GameContext, config: &Config, gl: &Gl) {
        // create vertex buffer
        self.quad_vbo = GlVertexBuffer::new(
            gl,
            gl::STATIC_DRAW,
            &[
                Vertex {
                    pos: vec2(-1.0, -1.0),
                    tex_coord: vec2(0.0, 1.0),
                },
                Vertex {
                    pos: vec2(1.0, -1.0),
                    tex_coord: vec2(1.0, 1.0),
                },
                Vertex {
                    pos: vec2(-1.0, 1.0),
                    tex_coord: vec2(0.0, 0.0),
                },
                Vertex {
                    pos: vec2(1.0, 1.0),
                    tex_coord: vec2(1.0, 0.0),
                },
            ],
        );

        // create index buffer
        self.quad_ibo = GlIndexBuffer::new(gl, gl::STATIC_DRAW, &[0, 1, 2, 3]);

        // create quad instance buffer
        self.quad_inbo = GlVertexBuffer::new(gl, gl::DYNAMIC_DRAW, &vec![Default::default(); MAX_INSTANCES]);

        // create quad vertex array object
        self.quad_vao = GlVertexArrayObject::new(gl);
        self.quad_vao.bind();
        self.quad_vao.bind_attrib(&self.quad_vbo, 0, 2, gl::FLOAT, gl::FALSE, 0 * size_of::<f32>(), 4 * size_of::<f32>(), 0); // pos
        self.quad_vao.bind_attrib(&self.quad_vbo, 1, 2, gl::FLOAT, gl::FALSE, 2 * size_of::<f32>(), 4 * size_of::<f32>(), 0); // tex_coord
        self.quad_vao.bind_attrib(&self.quad_inbo, 2, 2, gl::FLOAT, gl::FALSE, 0 * size_of::<f32>(), 8 * size_of::<f32>(), 1); // translate
        self.quad_vao.bind_attrib(&self.quad_inbo, 3, 2, gl::FLOAT, gl::FALSE, 2 * size_of::<f32>(), 8 * size_of::<f32>(), 1); // scale
        self.quad_vao.bind_attrib(&self.quad_inbo, 4, 1, gl::FLOAT, gl::FALSE, 4 * size_of::<f32>(), 8 * size_of::<f32>(), 1); // layer
        self.quad_vao.bind_attrib(&self.quad_inbo, 5, 1, gl::FLOAT, gl::FALSE, 5 * size_of::<f32>(), 8 * size_of::<f32>(), 1); // rotate
        self.quad_vao.bind_attrib(&self.quad_inbo, 6, 1, gl::FLOAT, gl::FALSE, 6 * size_of::<f32>(), 8 * size_of::<f32>(), 1); // tex_slot
        self.quad_vao.bind_attrib(&self.quad_inbo, 7, 1, gl::FLOAT, gl::FALSE, 7 * size_of::<f32>(), 8 * size_of::<f32>(), 1); // opacity
        self.quad_vao.unbind();

        // create gui instance buffer
        self.gui_inbo = GlVertexBuffer::new(gl, gl::DYNAMIC_DRAW, &vec![Default::default(); MAX_GUI_INSTANCES]);

        // create gui vertex array object
        self.gui_vao = GlVertexArrayObject::new(gl);
        self.gui_vao.bind();
        self.gui_vao.bind_attrib(&self.quad_vbo, 0, 2, gl::FLOAT, gl::FALSE, 0 * size_of::<f32>(), 4 * size_of::<f32>(), 0); // pos
        self.gui_vao.bind_attrib(&self.quad_vbo, 1, 2, gl::FLOAT, gl::FALSE, 2 * size_of::<f32>(), 4 * size_of::<f32>(), 0); // tex_coord
        self.gui_vao.bind_attrib(&self.gui_inbo, 2, 2, gl::FLOAT, gl::FALSE, 0 * size_of::<f32>(), 12 * size_of::<f32>(), 1); // translate
        self.gui_vao.bind_attrib(&self.gui_inbo, 3, 2, gl::FLOAT, gl::FALSE, 2 * size_of::<f32>(), 12 * size_of::<f32>(), 1); // scale
        self.gui_vao.bind_attrib(&self.gui_inbo, 4, 4, gl::FLOAT, gl::FALSE, 4 * size_of::<f32>(), 12 * size_of::<f32>(), 1); // color
        self.gui_vao.bind_attrib(&self.gui_inbo, 5, 1, gl::FLOAT, gl::FALSE, 8 * size_of::<f32>(), 12 * size_of::<f32>(), 1); // layer
        self.gui_vao.bind_attrib(&self.gui_inbo, 6, 1, gl::FLOAT, gl::FALSE, 9 * size_of::<f32>(), 12 * size_of::<f32>(), 1); // rotate
        self.gui_vao.bind_attrib(&self.gui_inbo, 7, 1, gl::FLOAT, gl::FALSE, 10 * size_of::<f32>(), 12 * size_of::<f32>(), 1); // tex_slot
        self.gui_vao.bind_attrib(&self.gui_inbo, 8, 1, gl::FLOAT, gl::FALSE, 11 * size_of::<f32>(), 12 * size_of::<f32>(), 1); // slice
        self.gui_vao.unbind();

        // create glyph instance buffer
        self.glyph_inbo = GlVertexBuffer::new(gl, gl::DYNAMIC_DRAW, &vec![Default::default(); MAX_GLYPH_INSTANCES]);

        // create glyph vertex array object
        self.glyph_vao = GlVertexArrayObject::new(gl);
        self.glyph_vao.bind();
        self.glyph_vao.bind_attrib(&self.quad_vbo, 0, 2, gl::FLOAT, gl::FALSE, 0 * size_of::<f32>(), 4 * size_of::<f32>(), 0); // pos
        self.glyph_vao.bind_attrib(&self.quad_vbo, 1, 2, gl::FLOAT, gl::FALSE, 2 * size_of::<f32>(), 4 * size_of::<f32>(), 0); // tex_coord
        self.glyph_vao.bind_attrib(&self.glyph_inbo, 2, 2, gl::FLOAT, gl::FALSE, 0 * size_of::<f32>(), 11 * size_of::<f32>(), 1); // translate
        self.glyph_vao.bind_attrib(&self.glyph_inbo, 3, 2, gl::FLOAT, gl::FALSE, 2 * size_of::<f32>(), 11 * size_of::<f32>(), 1); // scale
        self.glyph_vao.bind_attrib(&self.glyph_inbo, 4, 4, gl::FLOAT, gl::FALSE, 4 * size_of::<f32>(), 11 * size_of::<f32>(), 1); // color
        self.glyph_vao.bind_attrib(&self.glyph_inbo, 5, 1, gl::FLOAT, gl::FALSE, 8 * size_of::<f32>(), 11 * size_of::<f32>(), 1); // layer
        self.glyph_vao.bind_attrib(&self.glyph_inbo, 6, 1, gl::FLOAT, gl::FALSE, 9 * size_of::<f32>(), 11 * size_of::<f32>(), 1); // unicode
        self.glyph_vao
            .bind_attrib(&self.glyph_inbo, 7, 1, gl::FLOAT, gl::FALSE, 10 * size_of::<f32>(), 11 * size_of::<f32>(), 1); // width
        self.glyph_vao.unbind();

        // create uniform buffers
        self.quad_ubo = GlUniformBuffer::new(gl, gl::DYNAMIC_DRAW, &identity());
        self.gui_ubo = GlUniformBuffer::new(gl, gl::DYNAMIC_DRAW, &identity());
        self.glyph_ubo = GlUniformBuffer::new(gl, gl::DYNAMIC_DRAW, &identity());

        // create textures
        self.glyph_texture = create_font_texture(&config, gl);
        self.gui_textures = create_textures(ctx, gl, &GUI_TEXTURES);
        self.game_textures = create_textures(ctx, gl, &GAME_TEXTURES);

        // create shaders
        self.quad_shader = GlShader::new(gl, include_bytes!("glsl/quad.glslv"), include_bytes!("glsl/quad.glslf"));
        self.gui_shader = GlShader::new(gl, include_bytes!("glsl/gui.glslv"), include_bytes!("glsl/gui.glslf"));
        self.glyph_shader = GlShader::new(gl, include_bytes!("glsl/glyph.glslv"), include_bytes!("glsl/glyph.glslf"));

        // set default bindings
        unsafe {
            // culling
            gl.Enable(gl::CULL_FACE);
            gl.CullFace(gl::BACK);

            // blending
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            // depth
            gl.Disable(gl::DEPTH_TEST);
            gl.DepthMask(gl::FALSE);
            //gl.DepthFunc(gl::LESS);
        }

        // set context
        self.gl = Some(gl.clone());
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // update screen size
        self.resolution = vec2(width as f32, height as f32);
        println!("RESIZE: {:?}", &self.resolution);

        // update viewport
        let gl = self.gl.as_ref().expect("Missing OpenGL context");
        unsafe {
            gl.Viewport(0, 0, self.resolution.x as GLsizei, self.resolution.y as GLsizei);
        }
    }

    pub fn destroy(&mut self) {
        // clear context
        self.gl = None;

        // release textures
        self.glyph_texture.release();
        self.gui_textures.iter_mut().for_each(|texture| texture.release());
        self.game_textures.iter_mut().for_each(|texture| texture.release());

        // release shaders
        self.quad_shader.release();
        self.gui_shader.release();
        self.glyph_shader.release();

        // release vertex array objects
        self.quad_vao.release();
        self.gui_vao.release();
        self.glyph_vao.release();

        // release uniforms
        self.quad_ubo.release();
        self.gui_ubo.release();
        self.glyph_ubo.release();

        // release instance buffers
        self.quad_inbo.release();
        self.gui_inbo.release();
        self.glyph_inbo.release();

        // release const buffers
        self.quad_ibo.release();
        self.quad_vbo.release();
    }

    //////////////////////////////////////////////////
    // Clear functions

    pub fn clear(&mut self) {
        if let Some(gl) = self.gl.as_ref() {
            unsafe {
                gl.ClearColor(1.0, 0.2, 0.3, 1.0);
                gl.ClearDepthf(1.0);
                gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            }
        }
    }

    pub fn clear_depth(&mut self) {
        if let Some(gl) = self.gl.as_ref() {
            unsafe {
                gl.ClearDepthf(1.0);
                gl.Clear(gl::DEPTH_BUFFER_BIT);
            }
        }
    }

    //////////////////////////////////////////////////
    // Texture

    pub fn load_package_textures(&mut self, ctx: &GameContext, package_info: &PackageInfo) {
        if let Some(gl) = self.gl.as_ref() {
            // create texture map from package
            let package_textures: Vec<Vec<String>> = package_info
                .textures
                .iter()
                .map(|texture| texture.iter().map(|sub_tex| format!("level/{}/{}", &package_info.name, sub_tex)).collect())
                .collect();
            let package_texture_refs = package_textures
                .iter()
                .map(|texture_array| texture_array.iter().map(|texture| texture.as_str()).collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();
            self.package_textures = create_textures(ctx, gl, &package_texture_refs);
        }
    }

    pub fn unload_package_textures(&mut self) {
        self.package_textures.iter_mut().for_each(|texture| {
            texture.release();
        });
        self.package_textures.clear();
    }

    pub fn find_texture(&mut self, texture: TextureSrc) -> &mut GlTexture {
        match texture {
            TextureSrc::Font => &mut self.glyph_texture,
            TextureSrc::Gui(id) => self.gui_textures.get_mut(id).expect(&format!("Cannot find gui texture {}", id)),
            TextureSrc::Game(id) => self.game_textures.get_mut(id).expect(&format!("Cannot find game texture {}", id)),
            TextureSrc::Package(id) => self.package_textures.get_mut(id).expect(&format!("Cannot find package texture {}", id)),
        }
    }

    //////////////////////////////////////////////////
    // Getter

    pub fn resolution(&self) -> Vec2 {
        self.resolution
    }
}

//////////////////////////////////////////////////
// Trait implementation

unsafe impl Sync for GraphicsContext {}
unsafe impl Send for GraphicsContext {}

//////////////////////////////////////////////////
// Helper

fn create_textures(ctx: &GameContext, gl: &Gl, textures: &[Vec<&str>]) -> Vec<GlTexture> {
    textures
        .iter()
        .map(|pathes| {
            let images: Vec<RgbaImage> = pathes
                .iter()
                .map(|path| {
                    image::load_from_memory(&ctx.files().load_bytes(path).expect(&format!("Failed to load file {}", path)))
                        .expect("Failed to read memory")
                        .to_rgba8()
                })
                .collect();
            GlTexture::new(gl, &images)
        })
        .collect()
}

fn _create_color_texture(gl: &Gl, textures: &[U8Vec4]) -> GlTexture {
    let images: Vec<RgbaImage> = textures
        .iter()
        .map(|col| image::RgbaImage::from_vec(1, 1, vec![col.x, col.y, col.z, col.w]).expect(&format!("Failed to create color texture {}", col)))
        .collect();
    GlTexture::new(gl, &images)
}

fn create_font_texture(config: &Config, gl: &Gl) -> GlTexture {
    // create font
    let font = Font::try_from_bytes(&config.font).expect("Error constructing Font");
    let text: String = (0..128 as u8).map(|c| c as char).collect();
    let scale = Scale::uniform(config.font_size as f32);
    let v_metrics = font.v_metrics(scale);
    let glyphs = font.layout(&text, scale, point(0.0, v_metrics.ascent));
    // generate glyph images
    let images: Vec<GrayImage> = glyphs
        .map(|glyph| {
            // get bounding
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                let glyph_width = bounding_box.max.x - bounding_box.min.x;
                let offset = (config.font_size - glyph_width).max(0) / 2;
                // create new image to render glyph
                let mut image = GrayImage::new(glyph_width.max(config.font_size) as u32, config.font_size as u32);
                // Draw the glyph into the image per-pixel by using the draw closure
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + offset as u32,
                        y + bounding_box.min.y as u32,
                        // Turn the coverage into an alpha value
                        Luma([(v * 255.0) as u8]),
                    )
                });
                // Save the image to a png file
                resize(&image, config.font_size as u32, config.font_size as u32, FilterType::CatmullRom)
            } else {
                GrayImage::new(config.font_size as u32, config.font_size as u32)
            }
        })
        .collect();
    GlTexture::new(gl, &images)
}

#[test]
pub fn test_foo() {
    println!("{}", size_of::<GuiInstance>());
    println!("{}", size_of::<Vec2>());
    println!("{}", size_of::<f32>());
}
