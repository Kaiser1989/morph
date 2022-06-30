//////////////////////////////////////////////////
// Using

use game_gl::file::File;
use ini::Ini;
use lazy_static::*;
use nalgebra_glm::*;

//////////////////////////////////////////////////
// Constants

pub struct Config {
    pub font: Vec<u8>,
    pub font_size: i32,
    pub font_spacing: f32,

    pub menu_camera_zoom: f32,
    pub menu_layer: f32,
    pub menu_layer_delta: f32,
    pub menu_layer_font_offset: f32,

    pub level_camera_zoom: f32,
    pub level_camera_follow: f32,
    pub level_camera_speed: f32,
    pub level_camera_damping: f32,
    pub level_plane_far_layer: f32,
    pub level_plane_mid_layer: f32,
    pub level_plane_view_layer: f32,
    pub level_plane_near_layer: f32,
    pub level_morph_size: f32,
    pub level_target_size: f32,

    pub physic_group_metal: usize,
    pub physic_group_rubber: usize,
    pub physic_group_water: usize,
    pub physic_group_bubble: usize,
    pub physic_group_object: usize,
    pub physic_group_particle: usize,

    pub physic_grid_max_velocity: f32,
    pub physic_break_impulse: f32,

    pub morph_air_friction_metal: f32,
    pub morph_air_friction_rubber: f32,
    pub morph_air_friction_water: f32,
    pub morph_air_friction_bubble: f32,

    pub morph_ground_friction_metal: f32,
    pub morph_ground_friction_rubber: f32,
    pub morph_ground_friction_water: f32,
    pub morph_ground_friction_bubble: f32,

    pub morph_gravity_metal: f32,
    pub morph_gravity_rubber: f32,
    pub morph_gravity_water: f32,
    pub morph_gravity_bubble: f32,

    pub morph_angular_inertia_metal: f32,
    pub morph_angular_inertia_rubber: f32,
    pub morph_angular_inertia_water: f32,
    pub morph_angular_inertia_bubble: f32,

    pub morph_mass_metal: f32,
    pub morph_mass_rubber: f32,
    pub morph_mass_water: f32,
    pub morph_mass_bubble: f32,

    pub morph_bounce_metal: f32,
    pub morph_bounce_rubber: f32,
    pub morph_bounce_water: f32,
    pub morph_bounce_bubble: f32,

    pub morph_max_velocity_metal: f32,
    pub morph_max_velocity_rubber: f32,
    pub morph_max_velocity_water: f32,
    pub morph_max_velocity_bubble: f32,

    pub morph_max_angular_velocity_metal: f32,
    pub morph_max_angular_velocity_rubber: f32,
    pub morph_max_angular_velocity_water: f32,
    pub morph_max_angular_velocity_bubble: f32,

    pub morph_angular_damping_metal: f32,
    pub morph_angular_damping_rubber: f32,
    pub morph_angular_damping_water: f32,
    pub morph_angular_damping_bubble: f32,

    pub color_white: Vec4,
    pub color_red: Vec4,

    pub packages: Vec<String>,
}

//////////////////////////////////////////////////
// Level

lazy_static! {

    //////////////////////////////////////////////////
    // Ini Files
    static ref INI: Ini = Ini::load_from_str(&File::load_string("game.ini").expect("Failed to load 'game.ini'")).expect("Failed to parse 'game.ini'");

    static ref LEVEL_INI: Ini = Ini::load_from_str(&File::load_string("level/level.ini").expect("Failed to load level.ini")).expect("Failed to parse level.ini");

    //////////////////////////////////////////////////
    // Constants

    pub static ref CONFIG: Config = Config {

        font: File::load_bytes("game/font/font.ttf").expect("Failed to load font"),
        font_size: 64,
        font_spacing: 0.05,

        menu_camera_zoom: 10.0,
        menu_layer: 0.0,
        menu_layer_delta: 0.1,
        menu_layer_font_offset: -0.001,

        level_camera_zoom: 10.0,
        level_camera_follow: 0.1,
        level_camera_speed: 50.0,
        level_camera_damping: 5.0,
        level_plane_far_layer: 8.0,
        level_plane_mid_layer: 6.0,
        level_plane_view_layer: 4.0,
        level_plane_near_layer: 2.0,
        level_morph_size: 1.0,
        level_target_size: 1.5,

        physic_group_metal: 1,
        physic_group_rubber: 2,
        physic_group_water: 3,
        physic_group_bubble: 4,
        physic_group_object: 5,
        physic_group_particle: 6,

        physic_grid_max_velocity: 2.0,
        physic_break_impulse: 65.0,

        morph_air_friction_metal: read_from_ini("morph", "air_friction_metal", 0.03),
        morph_air_friction_rubber: read_from_ini("morph", "air_friction_rubber", 0.03),
        morph_air_friction_water: read_from_ini("morph", "air_friction_water", 0.03),
        morph_air_friction_bubble: read_from_ini("morph", "air_friction_bubble", 0.03),

        morph_ground_friction_metal: read_from_ini("morph", "ground_friction_metal", 0.5),
        morph_ground_friction_rubber: read_from_ini("morph", "ground_friction_rubber", 0.5),
        morph_ground_friction_water: read_from_ini("morph", "ground_friction_water", 0.5),
        morph_ground_friction_bubble: read_from_ini("morph", "ground_friction_bubble", 0.5),

        morph_gravity_metal: read_from_ini("morph", "gravity_metal", -9.81),
        morph_gravity_rubber: read_from_ini("morph", "gravity_rubber", -9.81),
        morph_gravity_water: read_from_ini("morph", "gravity_water", -9.81),
        morph_gravity_bubble: read_from_ini("morph", "gravity_bubble", 9.81),

        morph_angular_inertia_metal: read_from_ini("morph", "angular_inertia_metal", 1.0),
        morph_angular_inertia_rubber: read_from_ini("morph", "angular_inertia_rubber", 0.5),
        morph_angular_inertia_water: read_from_ini("morph", "angular_inertia_water", 0.0),
        morph_angular_inertia_bubble: read_from_ini("morph", "angular_inertia_bubble", 0.0),

        morph_mass_metal: read_from_ini("morph", "mass_metal", 150.0),
        morph_mass_rubber: read_from_ini("morph", "mass_rubber", 5.0),
        morph_mass_water: read_from_ini("morph", "mass_water", 1.0),
        morph_mass_bubble: read_from_ini("morph", "mass_bubble", 0.015),

        morph_bounce_metal: read_from_ini("morph", "bounce_metal", 0.35),
        morph_bounce_rubber: read_from_ini("morph", "bounce_rubber", 0.75),
        morph_bounce_water: read_from_ini("morph", "bounce_water", 0.0),
        morph_bounce_bubble: read_from_ini("morph", "bounce_bubble", 0.75),

        morph_max_velocity_metal: read_from_ini("morph", "max_velocity_metal", 20.0),
        morph_max_velocity_rubber: read_from_ini("morph", "max_velocity_rubber", 10.0),
        morph_max_velocity_water: read_from_ini("morph", "max_velocity_water", 15.0),
        morph_max_velocity_bubble: read_from_ini("morph", "max_velocity_bubble", 5.0),

        morph_max_angular_velocity_metal: read_from_ini("morph", "max_angular_velocity_metal", 20.0),
        morph_max_angular_velocity_rubber: read_from_ini("morph", "max_angular_velocity_rubber", 20.0),
        morph_max_angular_velocity_water: read_from_ini("morph", "max_angular_velocity_water", 20.0),
        morph_max_angular_velocity_bubble: read_from_ini("morph", "max_angular_velocity_bubble", 20.0),

        morph_angular_damping_metal: read_from_ini("morph", "angular_damping_metal", 1.0),
        morph_angular_damping_rubber: read_from_ini("morph", "angular_damping_rubber", 1.0),
        morph_angular_damping_water: read_from_ini("morph", "angular_damping_water", 1.0),
        morph_angular_damping_bubble: read_from_ini("morph", "angular_damping_bubble", 1.0),

        color_white: vec4(1.0, 1.0, 1.0, 1.0),
        color_red: vec4(1.0, 0.0, 0.0, 1.0),

        packages: load_level_packages(),
    };
}

//////////////////////////////////////////////////
// Helper

fn read_from_ini<T: std::fmt::Debug + std::str::FromStr>(section: &str, property: &str, default: T) -> T {
    let opt_section = if section.is_empty() { None } else { Some(section) };
    if let Some(sec) = INI.section(opt_section) {
        if let Some(prop) = sec.get(property) {
            if let Ok(x) = prop.parse::<T>() {
                return x;
            } else {
                print!("Failed to parse '{}' as '{}'. ", prop, std::any::type_name::<T>());
            }
        } else {
            print!("There is no property '{}' in section '{}'. ", property, section);
        }
    } else {
        print!("There is no section '{}'. ", section)
    }
    println!("Using default value '{:?}'.", default);
    default
}

fn load_level_packages() -> Vec<String> {
    LEVEL_INI.sections().filter_map(|x| x.map(|x| x.into())).collect()
}
