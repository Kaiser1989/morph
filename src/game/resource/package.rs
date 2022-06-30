//////////////////////////////////////////////////
// Using

use enum_map::*;
use game_gl::file::File;
use nalgebra_glm::*;
use serde::Deserialize;

use crate::game::ecs::resource::{MorphState, Plane, Role};

//////////////////////////////////////////////////
// Definition

#[derive(Default, Clone, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub textures: Vec<Vec<String>>,
    pub levels: Vec<LevelInfo>,
}

#[derive(Clone, Deserialize)]
pub struct LevelInfo {
    pub dimension: Vec2,
    pub available_morphs: EnumMap<MorphState, usize>,
    pub morph: MorphInfo,
    pub target: TargetInfo,
    pub objects: Vec<ObjectInfo>,
}

#[derive(Clone, Deserialize)]
pub struct MorphInfo {
    pub position: Vec2,
    pub state: MorphState,
    pub layer: u8,
}

#[derive(Clone, Deserialize)]
pub struct TargetInfo {
    pub position: Vec2,
    pub layer: u8,
}

#[derive(Clone, Deserialize)]
pub struct ObjectInfo {
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
    pub role: Role,
    pub texture: isize,
    pub texture_info: Option<TextureInfo>,
    pub block: Option<BlockInfo>,
    pub accelerator: Option<AcceleratorInfo>,
    pub breakable: Option<BreakableInfo>,
}

#[derive(Default, Clone, Deserialize)]
pub struct TextureInfo {
    pub layer: u8,
    pub plane: Plane,
    pub animation: f32,
}

#[derive(Default, Clone, Deserialize)]
pub struct BlockInfo {
    pub particles: Vec<usize>,
}

#[derive(Default, Clone, Deserialize)]
pub struct AcceleratorInfo {
    pub direction: AccelerationDirection,
    pub amplitude: f32,
    pub morph: EnumMap<MorphState, bool>,
}

#[derive(Default, Clone, Deserialize)]
pub struct BreakableInfo {
    pub group: u32,
}

#[derive(Copy, Clone, Deserialize)]
pub enum AccelerationDirection {
    Right,
    Left,
    Up,
    Down,
}

//////////////////////////////////////////////////
// Implementation

impl MorphInfo {
    pub fn new() -> MorphInfo {
        MorphInfo {
            position: Vec2::zeros(),
            layer: 0,
            state: MorphState::default(),
        }
    }
}

impl TargetInfo {
    pub fn new() -> TargetInfo {
        TargetInfo { position: Vec2::zeros(), layer: 0 }
    }
}

impl ObjectInfo {
    pub fn new() -> ObjectInfo {
        ObjectInfo {
            position: Vec2::zeros(),
            size: Vec2::zeros(),
            rotation: 0.0,
            role: Role::default(),
            texture: -1,
            texture_info: None,
            block: None,
            accelerator: None,
            breakable: None,
        }
    }
}

impl LevelInfo {
    pub fn new() -> LevelInfo {
        LevelInfo {
            dimension: Vec2::zeros(),
            available_morphs: EnumMap::default(),
            morph: MorphInfo::default(),
            target: TargetInfo::default(),
            objects: Vec::default(),
        }
    }
}

impl From<&str> for PackageInfo {
    fn from(package: &str) -> Self {
        serde_json::from_str(&File::load_string(&format!("level/{}/info.json", package)).expect("Failed to load level info file")).expect("Failed to parse json")
    }
}

impl Into<Vec2> for AccelerationDirection {
    fn into(self) -> Vec2 {
        match self {
            AccelerationDirection::Right => vec2(1.0, 0.0),
            AccelerationDirection::Left => vec2(-1.0, 0.0),
            AccelerationDirection::Up => vec2(0.0, 1.0),
            AccelerationDirection::Down => vec2(0.0, -1.0),
        }
    }
}

impl Default for MorphInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl Default for TargetInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl Default for ObjectInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl Default for LevelInfo {
    fn default() -> Self {
        Self::new()
    }
}
impl Default for AccelerationDirection {
    fn default() -> Self {
        Self::Right
    }
}
