//////////////////////////////////////////////////
// Using

use std::marker::PhantomData;

use specs::{System, World, WorldExt};

use crate::game::{
    config::Config,
    ecs::resource::GameTime,
    game_state::{GameStateEvent, GameSystem},
    Events, GraphicsContext, InputContext,
};

//////////////////////////////////////////////////
// Definition

pub struct SceneBuilder {
    update_systems: Vec<Box<dyn for<'a> GameSystem<'a>>>,
    render_systems: Vec<Box<dyn for<'a> GameSystem<'a>>>,
}

pub struct Scene<E: GameStateEvent> {
    config: Config,
    world: World,
    builder: SceneBuilder,
    _marker: PhantomData<E>,
}

//////////////////////////////////////////////////
// Implementation

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            update_systems: Vec::new(),
            render_systems: Vec::new(),
        }
    }

    pub fn update_system<S: for<'a> System<'a> + Default + 'static>(mut self) -> Self {
        self.update_systems.push(Box::new(S::default()));
        self
    }

    pub fn render_system<S: for<'a> System<'a> + Default + 'static>(mut self) -> Self {
        self.render_systems.push(Box::new(S::default()));
        self
    }
}

impl<E: GameStateEvent> Scene<E> {
    pub fn new(config: &Config) -> Self {
        let config = config.clone();
        let world = World::new();
        let builder = SceneBuilder::new();
        let _marker = PhantomData::default();
        Self { config, world, builder, _marker }
    }

    pub fn init(mut self, builder: SceneBuilder) -> Self {
        self.builder = builder;
        self.world = World::new();

        // init basic resources
        self.world.insert(GameTime::new(0.0, 0.0));

        // init systems
        self.builder.update_systems.iter_mut().for_each(|system| {
            system.init(&mut self.world);
        });

        // render systems
        self.builder.render_systems.iter_mut().for_each(|system| {
            system.init(&mut self.world);
        });

        self
    }

    pub fn cleanup(&mut self) {
        self.world = World::new();
        self.builder = SceneBuilder::new();
    }

    pub fn update(&mut self, elapsed_time: f32, events: &mut Events<E>) {
        // update time
        if let Some(game_time) = self.world.get_mut::<GameTime>() {
            game_time.update(elapsed_time);
        }

        // update systems
        self.builder.update_systems.iter_mut().for_each(|system| {
            system.update(&self.world);
        });

        // persist lazy updates, remove events
        self.world.maintain();
    }

    pub fn draw(&mut self, graphics: &GraphicsContext) {
        // make graphics context available for render systems
        self.world.insert(graphics.clone());

        // render systems
        self.builder.render_systems.iter_mut().for_each(|system| {
            system.update(&self.world);
        });

        // remove, so no other system can access it outside of render phase
        self.world.remove::<GraphicsContext>();
    }

    pub fn handle_input(&self, input: &InputContext, events: &mut Events<E>) {
        // TODO??
    }
}
