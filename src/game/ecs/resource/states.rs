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
    pub fn velocity_limit(&self, config: &Config) -> VelocityLimit {
        match self {
            MorphState::Metal => VelocityLimit::new(config.morph_max_velocity_metal, config.morph_max_angular_velocity_metal),
            MorphState::Rubber => VelocityLimit::new(config.morph_max_velocity_rubber, config.morph_max_angular_velocity_rubber),
            MorphState::Water => VelocityLimit::new(config.morph_max_velocity_water, config.morph_max_angular_velocity_water),
            MorphState::Bubble => VelocityLimit::new(config.morph_max_velocity_bubble, config.morph_max_angular_velocity_bubble),
        }
    }

    #[inline]
    pub fn velocity_damping(&self, config: &Config) -> VelocityDamping {
        match self {
            MorphState::Metal => VelocityDamping::new(config.morph_air_friction_metal, config.morph_angular_damping_metal),
            MorphState::Rubber => VelocityDamping::new(config.morph_air_friction_rubber, config.morph_angular_damping_rubber),
            MorphState::Water => VelocityDamping::new(config.morph_air_friction_water, config.morph_angular_damping_water),
            MorphState::Bubble => VelocityDamping::new(config.morph_air_friction_bubble, config.morph_angular_damping_bubble),
        }
    }

    #[inline]
    pub fn mass(&self, config: &Config) -> Mass {
        match self {
            MorphState::Metal => Mass::new(config.morph_mass_metal, config.morph_angular_inertia_metal),
            MorphState::Rubber => Mass::new(config.morph_mass_rubber, config.morph_angular_inertia_rubber),
            MorphState::Water => Mass::new(config.morph_mass_water, config.morph_angular_inertia_water),
            MorphState::Bubble => Mass::new(config.morph_mass_bubble, config.morph_angular_inertia_bubble),
        }
    }

    #[inline]
    pub fn gravity(&self, config: &Config) -> Gravity {
        match self {
            MorphState::Metal => Gravity::new(config.morph_gravity_metal),
            MorphState::Rubber => Gravity::new(config.morph_gravity_rubber),
            MorphState::Water => Gravity::new(config.morph_gravity_water),
            MorphState::Bubble => Gravity::new(config.morph_gravity_bubble),
        }
    }

    #[inline]
    pub fn material(&self, config: &Config) -> Material {
        match self {
            MorphState::Metal => Material::new(config.morph_bounce_metal, config.morph_ground_friction_metal),
            MorphState::Rubber => Material::new(config.morph_bounce_rubber, config.morph_ground_friction_rubber),
            MorphState::Water => Material::new(config.morph_bounce_water, config.morph_ground_friction_water),
            MorphState::Bubble => Material::new(config.morph_bounce_bubble, config.morph_ground_friction_bubble),
        }
    }

    #[inline]
    pub fn shape(&self, config: &Config) -> Shape {
        Shape::Ball(config.level_morph_size)
    }

    #[inline]
    pub fn collision(&self, config: &Config) -> Collision {
        Collision::new(
            match self {
                MorphState::Metal => config.physic_group_metal,
                MorphState::Rubber => config.physic_group_rubber,
                MorphState::Water => config.physic_group_water,
                MorphState::Bubble => config.physic_group_bubble,
            },
            vec![config.physic_group_object],
        )
    }

    #[inline]
    pub fn sensor(&self, config: &Config) -> Sensor {
        Sensor::new(
            match self {
                MorphState::Metal => config.physic_group_metal,
                MorphState::Rubber => config.physic_group_rubber,
                MorphState::Water => config.physic_group_water,
                MorphState::Bubble => config.physic_group_bubble,
            },
            vec![],
        )
    }

    #[inline]
    pub fn texture(&self, _config: &Config) -> Texture {
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
    pub fn shape(&self, config: &Config) -> Shape {
        match self {
            Role::Portal => Shape::Rect(vec2(config.level_target_size, config.level_target_size)),
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
    pub fn collision(&self, config: &Config) -> Collision {
        let (group, with) = match self {
            Role::Block => (
                config.physic_group_object,
                vec![
                    MorphState::Metal.collision(config).group,
                    MorphState::Rubber.collision(config).group,
                    MorphState::Water.collision(config).group,
                    MorphState::Bubble.collision(config).group,
                    config.physic_group_particle,
                ],
            ),
            Role::Breakable => (
                config.physic_group_object,
                vec![
                    MorphState::Metal.collision(config).group,
                    MorphState::Rubber.collision(config).group,
                    MorphState::Water.collision(config).group,
                    MorphState::Bubble.collision(config).group,
                    config.physic_group_particle,
                ],
            ),
            Role::Grid => (
                config.physic_group_object,
                vec![MorphState::Metal.collision(config).group, MorphState::Rubber.collision(config).group, config.physic_group_particle],
            ),
            Role::Particle => (config.physic_group_particle, vec![config.physic_group_object]),
            _ => (config.physic_group_object, vec![]),
        };
        Collision::new(group, with)
    }

    #[inline]
    pub fn sensor(&self, config: &Config) -> Sensor {
        Sensor::new(
            config.physic_group_object,
            match self {
                Role::Portal | Role::Court | Role::Accelerator => vec![
                    MorphState::Metal.sensor(config).group,
                    MorphState::Rubber.sensor(config).group,
                    MorphState::Water.sensor(config).group,
                    MorphState::Bubble.sensor(config).group,
                ],
                Role::Spikes => vec![MorphState::Rubber.sensor(config).group, MorphState::Bubble.sensor(config).group],
                Role::Grid => vec![MorphState::Water.sensor(config).group, MorphState::Bubble.sensor(config).group],
                _ => vec![],
            },
        )
    }
}

impl Plane {
    pub fn layer(&self, config: &Config) -> f32 {
        match &self {
            Plane::Far => config.level_plane_far_layer,
            Plane::Mid => config.level_plane_mid_layer,
            Plane::View => config.level_plane_view_layer,
            Plane::Near => config.level_plane_near_layer,
        }
    }

    pub fn values() -> &'static [Plane] {
        &[Plane::Far, Plane::Mid, Plane::View, Plane::Near]
    }
}
