//////////////////////////////////////////////////
// Using

use specs::{World, WorldExt};

use crate::game::{config::Config, game_state::GameStateEvent, Events, GraphicsContext, InputContext};

//////////////////////////////////////////////////
// Definition

pub struct SceneBuilder<E: GameStateEvent> {
    event: Option<E>,
}

pub struct Scene<E: GameStateEvent> {
    config: Config,
    world: World,
    builder: SceneBuilder<E>,
}

//////////////////////////////////////////////////
// Implementation

impl<E: GameStateEvent> SceneBuilder<E> {
    pub fn new() -> Self {
        Self { event: None }
    }
}

impl<E: GameStateEvent> Scene<E> {
    pub fn new(config: &Config) -> Self {
        let config = config.clone();
        let world = World::new();
        let builder = SceneBuilder::new();
        Self { config, world, builder }
    }

    pub fn init(&mut self, builder: SceneBuilder<E>) -> Self {
        self.builder = builder;
        self
    }

    pub fn cleanup(&mut self) {}

    pub fn update(&mut self, elapsed_time: f32, events: &mut Events<E>) {}

    pub fn draw(&self, graphics: &mut GraphicsContext) {}

    pub fn handle_input(&self, input: &InputContext, events: &mut Events<E>) {}
}
