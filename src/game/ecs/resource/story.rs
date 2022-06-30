//////////////////////////////////////////////////
// Using

use specs::prelude::*;

//////////////////////////////////////////////////
// Definition

#[derive(Debug, Default)]
pub struct Actors {
    pub morph: Option<Entity>,
    pub camera: Option<Entity>,
    pub portal: Option<Entity>,
}

#[derive(Debug, Default)]
pub struct Output {
    pub delay: f32,
    pub exit: bool,
    pub success: bool,
}

//////////////////////////////////////////////////
// Implementation

impl Actors {
    pub fn new(camera: Entity, morph: Entity, portal: Entity) -> Actors {
        Actors {
            camera: Some(camera),
            morph: Some(morph),
            portal: Some(portal),
        }
    }
}

impl Output {
    #[inline]
    pub fn success(&mut self, delay: f32) {
        self.delay = delay;
        self.exit = true;
        self.success = true;
    }

    #[inline]
    pub fn failure(&mut self, delay: f32) {
        self.delay = delay;
        self.exit = true;
        self.success = false;
    }
}
