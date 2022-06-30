//////////////////////////////////////////////////
// Using

use std::cmp::Ordering;
use std::collections::HashMap;
use std::default::Default;

use game_gl::gl;
use itertools::Itertools;
use lazy_static::*;
use nalgebra_glm::*;
use rusttype::{point, Font, Scale};
use shrev::Event;

use crate::game::config::*;
use crate::game::fx::*;
use crate::game::resource::*;

//////////////////////////////////////////////////
// Static Constants

lazy_static! {
    static ref FONT_WIDTHS: HashMap<char, f32> = create_font_widths();
}

//////////////////////////////////////////////////
// Definition

pub struct Gui<T: Event + Clone> {
    dimension: Vec2,
    resolution: Vec2,
    builder: GuiBuilder<T>,
}

pub const HORIZONTAL: usize = 0;
pub const VERTICAL: usize = 1;
pub const CENTER: usize = 2;
pub const LEFT: usize = 3;
pub const RIGHT: usize = 4;
pub const TOP: usize = 5;
pub const BOTTOM: usize = 6;

#[derive(Debug, Copy, Clone)]
pub enum Value {
    Auto,
    Fixed(f32),
}

#[derive(Default, Copy, Clone)]
pub struct Space {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

#[derive(Clone)]
pub struct GuiBuilder<T: Event + Clone> {
    id: &'static str,
    pos: Vec2,
    size: Vec2,
    auto_size: BVec2,
    layer: f32,
    padding: Space,
    margin: Space,
    align: usize,
    x_align: usize,
    y_align: usize,
    children: Vec<GuiBuilder<T>>,
    text: Option<(String, f32, Vec4)>,
    glyph: Option<(char, f32, Vec4)>,
    texture: Option<(TextureSrc, usize)>,
    color: Option<Vec4>,
    rounded: Option<f32>,
    click_event: Option<T>,
    fast_click_event: Option<T>,
}

pub struct GuiRenderInfo {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Vec4,
    pub layer: f32,
    pub texture: TextureSrc,
    pub texture_slot: usize,
    pub slice: f32,
    pub radius: f32,
}

pub struct GuiFontRenderInfo {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Vec4,
    pub layer: f32,
    pub unicode: char,
    pub width: f32,
}

//////////////////////////////////////////////////
// Implementation

impl<T: Event + Clone> Gui<T> {
    pub fn new() -> Gui<T> {
        Gui::<T> {
            dimension: vec2(0.0, 0.0),
            resolution: vec2(0.0, 0.0),
            builder: GuiBuilder::<T>::new("root"),
        }
    }

    pub fn init(&mut self, builder: &GuiBuilder<T>) {
        self.builder = builder.clone();
        // set pos/size of root element
        self.builder.pos = vec2(0.0, self.dimension.y);
        self.builder.size = self.dimension;
        self.builder.layer = CONFIG.menu_layer;
    }

    pub fn update(&mut self) {
        update(&mut self.builder);
    }

    pub fn handle_input(&self, input: &InputContext, events: &mut Events<T>) {
        if let Some(position) = input.fast_click() {
            let gui_pos = vec2(position.x * self.dimension.x, position.y * self.dimension.y);
            for event in fast_click_element(&self.builder, gui_pos).into_iter() {
                events.write(event);
            }
        }
        if let Some(position) = input.click() {
            let gui_pos = vec2(position.x * self.dimension.x, position.y * self.dimension.y);
            for event in click_element(&self.builder, gui_pos).into_iter() {
                events.write(event);
            }
        }
    }

    pub fn find_mut(&mut self, id: &str) -> Option<&mut GuiBuilder<T>> {
        find_element(&mut self.builder, id)
    }

    pub fn adjust_dimension(&mut self, resolution: Vec2) {
        // dimension
        self.resolution = resolution;
        let aspect_ratio = resolution.x / resolution.y;
        let aspect_vec = if aspect_ratio > 1.0 { vec2(aspect_ratio, 1.0) } else { vec2(1.0, 1.0 / aspect_ratio) };
        self.dimension = aspect_vec * CONFIG.menu_camera_zoom;
        // change pos/size of root element
        self.builder.pos = vec2(0.0, self.dimension.y);
        self.builder.size = self.dimension;
        // update gui
        self.update();
    }

    pub fn draw(&self, graphics: &mut GraphicsContext) {
        // clear depth
        graphics.clear_depth();

        // update viewproj
        let view = look_at_rh(&Vec3::z(), &Vec3::zeros(), &Vec3::y());
        let proj = ortho_rh(0.0, self.dimension.x, 0.0, self.dimension.y, 0.1, 5.0);
        let view_proj = proj * view;

        // create instances; sort by layer -> texture
        let mut instances: Vec<(TextureSrc, GuiInstance)> = collect_render_data(&self.builder)
            .into_iter()
            .map(|gui_data| {
                let instance = GuiInstance {
                    translate: gui_data.position,
                    scale: gui_data.size,
                    color: gui_data.color,
                    layer: gui_data.layer,
                    tex_slot: gui_data.texture_slot as f32,
                    slice: gui_data.slice,
                    radius: gui_data.radius,
                };
                (gui_data.texture, instance)
            })
            .collect();
        instances.sort_unstable_by(|(t0, i0), (t1, i1)| match i0.layer.partial_cmp(&i1.layer).unwrap() {
            Ordering::Equal => t0.cmp(t1),
            x => x,
        });

        // create glyph instances; sort by layer
        let mut glyph_instances: Vec<GlyphInstance> = collect_font_render_data(&self.builder)
            .into_iter()
            .map(|gui_data| GlyphInstance {
                translate: gui_data.position.into(),
                scale: gui_data.size.into(),
                layer: gui_data.layer,
                color: gui_data.color.into(),
                unicode: gui_data.unicode as u8 as f32,
                width: gui_data.width,
            })
            .collect();
        glyph_instances.sort_unstable_by(|gi0, gi1| gi0.layer.partial_cmp(&gi1.layer).unwrap());

        // bind shader
        graphics.gui_shader.bind();
        graphics.gui_shader.link_texture(1, "t_textures");
        graphics.gui_shader.link_texture(2, "t_slices");
        graphics.gui_shader.link_uniform(1, "Locals");
        // bind vao
        graphics.gui_vao.bind();
        // bind index buffer
        graphics.quad_ibo.bind();
        // bind uniforms
        graphics.gui_ubo.update(&view_proj);
        graphics.gui_ubo.bind(1);
        // bind slice textures
        graphics.find_texture(TEX_GUI_SLICE).bind(2);

        // render by texture
        for (texture, v) in &instances.into_iter().group_by(|(t, _)| *t) {
            // bind instances
            let instances: Vec<GuiInstance> = v.into_iter().map(|(_, i)| i).collect();
            graphics.gui_inbo.update(&instances);
            // bind textures
            graphics.find_texture(texture).bind(1);

            // draw
            graphics.gui_shader.draw_elements_instanced(gl::TRIANGLE_STRIP, graphics.quad_ibo.count(), instances.len());

            // unbind textures
            graphics.find_texture(texture).unbind();
        }

        // unbind all
        graphics.find_texture(TEX_GUI_SLICE).unbind();
        graphics.quad_ibo.unbind();
        graphics.gui_ubo.unbind();
        graphics.gui_vao.unbind();
        graphics.gui_shader.unbind();

        // bind shader
        graphics.glyph_shader.bind();
        graphics.glyph_shader.link_texture(2, "t_glyphs");
        graphics.glyph_shader.link_uniform(1, "Locals");
        // bind vao
        graphics.glyph_vao.bind();
        // bind index buffer
        graphics.quad_ibo.bind();
        // bind uniforms
        graphics.glyph_ubo.update(&view_proj);
        graphics.glyph_ubo.bind(1);
        // bind instances
        graphics.glyph_inbo.update(&glyph_instances);
        // bind glyph texture
        graphics.glyph_texture.bind(2);

        // draw
        graphics.glyph_shader.draw_elements_instanced(gl::TRIANGLE_STRIP, graphics.quad_ibo.count(), glyph_instances.len());

        // unbind all
        graphics.glyph_texture.unbind();
        graphics.glyph_ubo.unbind();
        graphics.quad_ibo.unbind();
        graphics.glyph_vao.unbind();
        graphics.glyph_shader.unbind();
    }
}

impl<T: Event + Clone> GuiBuilder<T> {
    pub fn new(id: &'static str) -> GuiBuilder<T> {
        GuiBuilder {
            id,
            pos: Vec2::zeros(),
            size: Vec2::zeros(),
            auto_size: BVec2::new(false, false),
            layer: 0.0,
            padding: Space::default(),
            margin: Space::default(),
            align: 0,
            x_align: 0,
            y_align: 0,
            children: Vec::default(),
            text: None,
            glyph: None,
            texture: None,
            color: None,
            rounded: None,
            click_event: None,
            fast_click_event: None,
        }
    }

    pub fn id(&self) -> &str {
        self.id
    }

    pub fn size(mut self, width: Value, height: Value) -> GuiBuilder<T> {
        self.auto_size.x = if let Value::Auto = width { true } else { false };
        self.auto_size.y = if let Value::Auto = height { true } else { false };
        self.size.x = if let Value::Fixed(x) = width { x } else { 0.0 };
        self.size.y = if let Value::Fixed(y) = height { y } else { 0.0 };
        self
    }

    pub fn padding(mut self, left: f32, right: f32, bottom: f32, top: f32) -> GuiBuilder<T> {
        self.padding = Space { left, right, bottom, top };
        self
    }

    pub fn margin(mut self, left: f32, right: f32, bottom: f32, top: f32) -> GuiBuilder<T> {
        self.margin = Space { left, right, bottom, top };
        self
    }

    pub fn vertical(mut self) -> GuiBuilder<T> {
        self.align = VERTICAL;
        self
    }

    pub fn flow(mut self, value: usize) -> GuiBuilder<T> {
        self.align = value;
        self
    }

    pub fn align(mut self, x_align: usize, y_align: usize) -> GuiBuilder<T> {
        self.x_align = x_align;
        self.y_align = y_align;
        self
    }

    pub fn child(mut self, value: GuiBuilder<T>) -> GuiBuilder<T> {
        self.children.push(value);
        self
    }

    pub fn children(mut self, value: Vec<GuiBuilder<T>>) -> GuiBuilder<T> {
        self.children.extend(value);
        self
    }

    pub fn texture(mut self, texture: TextureSrc, slot: usize) -> GuiBuilder<T> {
        self.texture = Some((texture, slot));
        self
    }

    pub fn color(mut self, r: f32, g: f32, b: f32, a: f32) -> GuiBuilder<T> {
        self.color = Some(vec4(r, g, b, a));
        self
    }

    pub fn rounded(mut self, radius: f32) -> GuiBuilder<T> {
        self.rounded = Some(radius);
        self
    }

    pub fn text(mut self, text: &str, size: f32, color: Vec4) -> GuiBuilder<T> {
        self.text = Some((text.into(), size, color));
        self
    }

    pub fn click(mut self, event: T) -> GuiBuilder<T> {
        self.click_event = Some(event);
        self
    }

    pub fn fast_click(mut self, event: T) -> GuiBuilder<T> {
        self.fast_click_event = Some(event);
        self
    }

    pub fn change_text(&mut self, text: &str) {
        if let Some(text_info) = &mut self.text {
            text_info.0 = text.into()
        }
    }

    pub fn change_text_color(&mut self, color: Vec4) {
        if let Some(text_info) = &mut self.text {
            text_info.2 = color;
        }
    }
}

//////////////////////////////////////////////////
// Helper

fn update<T: Event + Clone>(element: &mut GuiBuilder<T>) {
    // get element data
    let pos = element.pos;
    let size = element.size;
    let padding = element.padding;
    let layer = element.layer;

    // check for text node
    if let Some((text, size, color)) = &element.text {
        element.children = text
            .chars()
            .map(|c| {
                let char_width = *FONT_WIDTHS.get(&c).unwrap();
                let mut child =
                    GuiBuilder::<T>::new("font_do_not_search_for")
                        .size(Value::Fixed(size * char_width), Value::Fixed(*size))
                        .margin(size * CONFIG.font_spacing, size * CONFIG.font_spacing, 0.0, 0.0);
                child.glyph = Some((c, char_width, *color));
                child
            })
            .collect();
    }

    // reset autosized children
    element.children.iter_mut().for_each(|child| {
        if child.auto_size.x {
            child.size.x = 0.0;
        }
        if child.auto_size.y {
            child.size.y = 0.0;
        }
    });

    // set childrens position
    match element.align {
        HORIZONTAL => {
            // calc x range
            let mut x_left = pos.x + padding.left;
            let x_right = pos.x + size.x - padding.right;

            // check fill size elements
            let count = element.children.iter_mut().filter(|child| child.auto_size.x).count();
            if count > 0 {
                let fill_x = (x_right - x_left - element.children.iter().map(|child| child.margin.left + child.size.x + child.margin.right).sum::<f32>()) / count as f32;
                element.children.iter_mut().filter(|child| child.auto_size.x).for_each(|child| {
                    child.size.x = fill_x;
                });
            }

            // iterate over children and set position
            element.children.iter_mut().for_each(|child| {
                // set child pos
                x_left += child.margin.left;
                child.pos.x = x_left;
                x_left += child.size.x + child.margin.right;
            });

            // set y to children
            let y_align = element.y_align;
            element.children.iter_mut().for_each(|child| {
                // calc y range
                let mut y_top = pos.y - padding.top;
                let y_bottom = pos.y - size.y + padding.bottom;

                // set child pos
                y_top -= child.margin.top;
                child.pos.y = y_top;
                y_top -= child.size.y + child.margin.bottom;

                // calc remaining space
                let remain = y_top - y_bottom;
                if child.auto_size.y {
                    child.size.y = remain;
                } else {
                    match y_align {
                        CENTER => {
                            child.pos.y -= remain * 0.5;
                        }
                        BOTTOM => {
                            child.pos.y -= remain;
                        }
                        _ => {} // nothing to do
                    }
                }
            });

            // calc remaining space
            let remain = x_right - x_left;
            match element.x_align {
                CENTER => {
                    element.children.iter_mut().for_each(|child| child.pos.x += remain * 0.5);
                }
                RIGHT => {
                    element.children.iter_mut().for_each(|child| child.pos.x += remain);
                }
                _ => {} // nothing to do
            }
        }
        VERTICAL => {
            // calc y range
            let mut y_top = pos.y - padding.top;
            let y_bottom = pos.y - size.y + padding.bottom;

            // check fill size elements
            let count = element.children.iter_mut().filter(|child| child.auto_size.y).count();
            if count > 0 {
                let fill_y = (y_top - y_bottom - element.children.iter().map(|child| child.margin.top + child.size.y + child.margin.bottom).sum::<f32>()) / count as f32;
                element.children.iter_mut().filter(|child| child.auto_size.y).for_each(|child| {
                    child.size.y = fill_y;
                });
            }

            // iterate over children and set position
            element.children.iter_mut().for_each(|child| {
                // set child pos
                y_top -= child.margin.top;
                child.pos.y = y_top;
                y_top -= child.size.y + child.margin.bottom;
            });

            // get start values for x
            let x_align = element.x_align;
            element.children.iter_mut().for_each(|child| {
                // calc y range
                let mut x_left = pos.x + padding.left;
                let x_right = pos.x + size.x - padding.right;

                // set child pos
                x_left += child.margin.left;
                child.pos.x = x_left;
                x_left += child.size.x + child.margin.right;

                // calc remaining space
                let remain = x_right - x_left;
                if child.auto_size.x {
                    child.size.x = remain;
                } else {
                    match x_align {
                        CENTER => {
                            child.pos.x += remain * 0.5;
                        }
                        RIGHT => {
                            child.pos.x += remain;
                        }
                        _ => {} // nothing to do
                    }
                }
            });

            // calc remaining space
            let remain = y_top - y_bottom;
            match element.y_align {
                CENTER => {
                    element.children.iter_mut().for_each(|child| child.pos.y -= remain * 0.5);
                }
                BOTTOM => {
                    element.children.iter_mut().for_each(|child| child.pos.y -= remain);
                }
                _ => {} // nothing to do
            }
        }
        _ => {}
    }

    // update all children
    element.children.iter_mut().for_each(|child| {
        // set childrens' layer
        child.layer = layer + CONFIG.menu_layer_delta;

        // recursive update of all children
        update(child);
    });
}

fn collect_render_data<T: Event + Clone>(element: &GuiBuilder<T>) -> Vec<GuiRenderInfo> {
    let mut data: Vec<GuiRenderInfo> = Vec::new();

    // get data of this element
    if let Some((texture, texture_slot)) = &element.texture {
        // add center texture
        let position = vec2(element.pos.x + element.size.x * 0.5, element.pos.y - element.size.y * 0.5);
        let size = element.size * 0.5;
        let layer = element.layer;
        let texture = *texture;
        let texture_slot = *texture_slot;
        let color = element.color.unwrap_or_else(|| vec4(1.0, 1.0, 1.0, 1.0));
        let radius = element.rounded.unwrap_or(0.0);
        match element.rounded {
            Some(_) => (0..9),
            None => (4..5),
        }
        .for_each(|i| {
            data.push(GuiRenderInfo::new(position, size, color, layer, texture, texture_slot, i as f32, radius));
        });
    }

    // get data of children
    data.extend(element.children.iter().map(|child| collect_render_data(child)).flatten());

    data
}

fn collect_font_render_data<T: Event + Clone>(element: &GuiBuilder<T>) -> Vec<GuiFontRenderInfo> {
    let mut data: Vec<GuiFontRenderInfo> = Vec::new();

    // get text data of this element
    if let Some((glyph, width, color)) = &element.glyph {
        let position = vec2(element.pos.x + element.size.x * 0.5, element.pos.y - element.size.y * 0.5);
        let size = vec2(element.size.x * 0.5, element.size.y * 0.5);
        let color = *color;
        let layer = element.layer - CONFIG.menu_layer_font_offset;
        let unicode = *glyph;
        let width = *width;
        data.push(GuiFontRenderInfo {
            position,
            size,
            color,
            layer,
            unicode,
            width,
        });
    }

    // get data of children
    data.extend(element.children.iter().map(|child| collect_font_render_data(child)).flatten());

    data
}

fn click_element<T: Event + Clone>(element: &GuiBuilder<T>, click: Vec2) -> Vec<T> {
    let mut events: Vec<T> = Vec::new();

    // get events this element
    if let Some(event) = element.click_event.as_ref() {
        // calc bounding rect
        let min_pos = vec2(element.pos.x, element.pos.y - element.size.y);
        let max_pos = vec2(element.pos.x + element.size.x, element.pos.y);
        // check click collision
        if inside_rect(min_pos, max_pos, click) {
            events.push(event.clone());
        }
    }

    // get events of children
    events.extend(element.children.iter().map(|child| click_element(child, click)).flatten());

    events
}

fn fast_click_element<T: Event + Clone>(element: &GuiBuilder<T>, click: Vec2) -> Vec<T> {
    let mut events: Vec<T> = Vec::new();

    // get events this element
    if let Some(event) = element.fast_click_event.as_ref() {
        // calc bounding rect
        let min_pos = vec2(element.pos.x, element.pos.y - element.size.y);
        let max_pos = vec2(element.pos.x + element.size.x, element.pos.y);
        // check click collision
        if inside_rect(min_pos, max_pos, click) {
            events.push(event.clone());
        }
    }

    // get events of children
    events.extend(element.children.iter().map(|child| fast_click_element(child, click)).flatten());

    events
}

fn find_element<'a, T: Event + Clone>(element: &'a mut GuiBuilder<T>, id: &str) -> Option<&'a mut GuiBuilder<T>> {
    if element.id == id {
        Some(element)
    } else {
        element.children.iter_mut().find_map(|child| find_element(child, id))
    }
}

fn inside_rect(min: Vec2, max: Vec2, point: Vec2) -> bool {
    point.x >= min.x && point.x <= max.x && point.y >= min.y && point.y <= max.y
}

pub fn create_font_widths() -> HashMap<char, f32> {
    // create font
    let font = Font::try_from_bytes(&CONFIG.font).expect("Error constructing Font");
    let text: String = (0..128 as u8).map(|c| c as char).collect();
    let scale = Scale::uniform(CONFIG.font_size as f32);
    let v_metrics = font.v_metrics(scale);
    let glyphs = font.layout(&text, scale, point(0.0, v_metrics.ascent));
    glyphs
        .enumerate()
        .map(|(i, glyph)| {
            // get width of glyph (as percentage)
            let width = if let Some(bounding_box) = glyph.pixel_bounding_box() {
                let glyph_width = (bounding_box.max.x - bounding_box.min.x).min(CONFIG.font_size);
                glyph_width as f32 / CONFIG.font_size as f32
            } else {
                0.25
            };
            (i as u8 as char, width)
        })
        .collect()
}

impl GuiRenderInfo {
    pub fn new(position: Vec2, size: Vec2, color: Vec4, layer: f32, texture: TextureSrc, texture_slot: usize, slice: f32, radius: f32) -> GuiRenderInfo {
        GuiRenderInfo {
            position,
            size,
            color,
            layer,
            texture,
            texture_slot,
            slice,
            radius,
        }
    }
}
