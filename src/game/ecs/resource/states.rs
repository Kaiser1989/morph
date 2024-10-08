//////////////////////////////////////////////////
// Using

use enum_iterator::*;
use enum_map::Enum;
use nalgebra_glm::*;
use serde::Deserialize;

use crate::game::config::*;
use crate::game::ecs::component::*;
use crate::game::fx::*;

//////////////////////////////////////////////////
// Definition

#[derive(Debug, Copy, Clone, PartialEq, Enum, Deserialize, Sequence)]
pub enum MorphState {
    Metal,
    Rubber,
    Water,
    Bubble,
}
impl Default for MorphState {
    fn default() -> Self {
        MorphState::Rubber
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub enum Role {
    None,
    Block,
    Portal,
    Spikes,
    Breakable,
    Grid,
    Accelerator,
    Court,
    Particle,
    Morph,
}
impl Default for Role {
    fn default() -> Self {
        Role::None
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Enum, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Plane {
    // TODO: move to fx
    Far,
    Mid,
    View,
    Near,
}
impl Default for Plane {
    fn default() -> Self {
        Self::View
    }
}

//////////////////////////////////////////////////
// Implementation

impl MorphState {
    #[inline]
    pub fn to_string(&self) -> &'static str {
        match self {
            MorphState::Metal => "metal",
            MorphState::Rubber => "rubber",
            MorphState::Water => "water",
            MorphState::Bubble => "bubble",
        }
    }

    #[inline]
    pub fn velocity_limit(&self) -> VelocityLimit {
        match self {
            MorphState::Metal => VelocityLimit::new(CONFIG.morph_max_velocity_metal, CONFIG.morph_max_angular_velocity_metal),
            MorphState::Rubber => VelocityLimit::new(CONFIG.morph_max_velocity_rubber, CONFIG.morph_max_angular_velocity_rubber),
            MorphState::Water => VelocityLimit::new(CONFIG.morph_max_velocity_water, CONFIG.morph_max_angular_velocity_water),
            MorphState::Bubble => VelocityLimit::new(CONFIG.morph_max_velocity_bubble, CONFIG.morph_max_angular_velocity_bubble),
        }
    }

    #[inline]
    pub fn velocity_damping(&self) -> VelocityDamping {
        match self {
            MorphState::Metal => VelocityDamping::new(CONFIG.morph_air_friction_metal, CONFIG.morph_angular_damping_metal),
            MorphState::Rubber => VelocityDamping::new(CONFIG.morph_air_friction_rubber, CONFIG.morph_angular_damping_rubber),
            MorphState::Water => VelocityDamping::new(CONFIG.morph_air_friction_water, CONFIG.morph_angular_damping_water),
            MorphState::Bubble => VelocityDamping::new(CONFIG.morph_air_friction_bubble, CONFIG.morph_angular_damping_bubble),
        }
    }

    #[inline]
    pub fn mass(&self) -> Mass {
        match self {
            MorphState::Metal => Mass::new(CONFIG.morph_mass_metal, CONFIG.morph_angular_inertia_metal),
            MorphState::Rubber => Mass::new(CONFIG.morph_mass_rubber, CONFIG.morph_angular_inertia_rubber),
            MorphState::Water => Mass::new(CONFIG.morph_mass_water, CONFIG.morph_angular_inertia_water),
            MorphState::Bubble => Mass::new(CONFIG.morph_mass_bubble, CONFIG.morph_angular_inertia_bubble),
        }
    }

    #[inline]
    pub fn gravity(&self) -> Gravity {
        match self {
            MorphState::Metal => Gravity::new(CONFIG.morph_gravity_metal),
            MorphState::Rubber => Gravity::new(CONFIG.morph_gravity_rubber),
            MorphState::Water => Gravity::new(CONFIG.morph_gravity_water),
            MorphState::Bubble => Gravity::new(CONFIG.morph_gravity_bubble),
        }
    }

    #[inline]
    pub fn material(&self) -> Material {
        match self {
            MorphState::Metal => Material::new(CONFIG.morph_bounce_metal, CONFIG.morph_ground_friction_metal),
            MorphState::Rubber => Material::new(CONFIG.morph_bounce_rubber, CONFIG.morph_ground_friction_rubber),
            MorphState::Water => Material::new(CONFIG.morph_bounce_water, CONFIG.morph_ground_friction_water),
            MorphState::Bubble => Material::new(CONFIG.morph_bounce_bubble, CONFIG.morph_ground_friction_bubble),
        }
    }

    #[inline]
    pub fn shape(&self) -> Shape {
        Shape::Ball(CONFIG.level_morph_size)
    }

    #[inline]
    pub fn collision(&self) -> Collision {
        Collision::new(
            match self {
                MorphState::Metal => CONFIG.physic_group_metal,
                MorphState::Rubber => CONFIG.physic_group_rubber,
                MorphState::Water => CONFIG.physic_group_water,
                MorphState::Bubble => CONFIG.physic_group_bubble,
            },
            vec![CONFIG.physic_group_object],
        )
    }

    #[inline]
    pub fn sensor(&self) -> Sensor {
        Sensor::new(
            match self {
                MorphState::Metal => CONFIG.physic_group_metal,
                MorphState::Rubber => CONFIG.physic_group_rubber,
                MorphState::Water => CONFIG.physic_group_water,
                MorphState::Bubble => CONFIG.physic_group_bubble,
            },
            vec![],
        )
    }

    #[inline]
    pub fn texture(&self) -> Texture {
        match self {
            MorphState::Metal => Texture::new(TEX_GAME_METAL),
            MorphState::Rubber => Texture::new(TEX_GAME_RUBBER),
            MorphState::Water => Texture::new(TEX_GAME_WATER),
            MorphState::Bubble => Texture::new(TEX_GAME_BUBBLE),
        }
    }
}

impl Role {
    #[inline]
    pub fn shape(&self) -> Shape {
        match self {
            Role::Portal => Shape::Rect(vec2(CONFIG.level_target_size, CONFIG.level_target_size)),
            _ => unimplemented!("Create it yourself"),
        }
    }

    #[inline]
    pub fn texture(&self) -> Texture {
        match self {
            Role::Portal => Texture::new(TEX_GAME_PORTAL),
            _ => unimplemented!("Create it yourself"),
        }
    }

    #[inline]
    pub fn collision(&self) -> Collision {
        let (group, with) = match self {
            Role::Block => (
                CONFIG.physic_group_object,
                vec![
                    MorphState::Metal.collision().group,
                    MorphState::Rubber.collision().group,
                    MorphState::Water.collision().group,
                    MorphState::Bubble.collision().group,
                    CONFIG.physic_group_particle,
                ],
            ),
            Role::Breakable => (
                CONFIG.physic_group_object,
                vec![
                    MorphState::Metal.collision().group,
                    MorphState::Rubber.collision().group,
                    MorphState::Water.collision().group,
                    MorphState::Bubble.collision().group,
                    CONFIG.physic_group_particle,
                ],
            ),
            Role::Grid => (
                CONFIG.physic_group_object,
                vec![MorphState::Metal.collision().group, MorphState::Rubber.collision().group, CONFIG.physic_group_particle],
            ),
            Role::Particle => (CONFIG.physic_group_particle, vec![CONFIG.physic_group_object]),
            _ => (CONFIG.physic_group_object, vec![]),
        };
        Collision::new(group, with)
    }

    #[inline]
    pub fn sensor(&self) -> Sensor {
        Sensor::new(
            CONFIG.physic_group_object,
            match self {
                Role::Portal | Role::Court | Role::Accelerator => vec![
                    MorphState::Metal.sensor().group,
                    MorphState::Rubber.sensor().group,
                    MorphState::Water.sensor().group,
                    MorphState::Bubble.sensor().group,
                ],
                Role::Spikes => vec![MorphState::Rubber.sensor().group, MorphState::Bubble.sensor().group],
                Role::Grid => vec![MorphState::Water.sensor().group, MorphState::Bubble.sensor().group],
                _ => vec![],
            },
        )
    }
}

impl Plane {
    pub fn layer(&self) -> f32 {
        match &self {
            Plane::Far => CONFIG.level_plane_far_layer,
            Plane::Mid => CONFIG.level_plane_mid_layer,
            Plane::View => CONFIG.level_plane_view_layer,
            Plane::Near => CONFIG.level_plane_near_layer,
        }
    }

    pub fn values() -> &'static [Plane] {
        &[Plane::Far, Plane::Mid, Plane::View, Plane::Near]
    }
}
