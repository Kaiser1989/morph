//////////////////////////////////////////////////
// Modules

pub mod config;
pub mod ecs;
pub mod fx;
pub mod resource;
pub mod state;
pub mod utils;

//////////////////////////////////////////////////
// Using

use std::sync::Arc;

use config::Config;
use config::RawConfig;
use game_gl::prelude::*;
use game_state::GameState;
use log::info;
use shrev::ReaderId;

use crate::game::fx::*;
use crate::game::resource::*;
use crate::game::state::*;

//////////////////////////////////////////////////
// Definition

pub struct GameManager {
    states: Vec<Box<dyn GameState>>,
    events: Events<StateEvent>,
    reader: ReaderId<StateEvent>,
    resource: ResourceContext,
    graphics: GraphicsContext,
    input: InputContext,
    config: Config,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StateEvent {
    Menu,
    MenuSettings,
    MenuPackage,
    MenuPackageLevel,
    Level,
    LevelPause,
    LevelSuccess,
    LevelFailure,
    Back,
    Exit,

    LoadPackage(String),
    UnloadPackage,
    LoadLevel(usize),
    LoadLevelNext,
    UnloadLevel,
}

//////////////////////////////////////////////////
// Implementation

impl GameLoop for GameManager {
    fn title(&self) -> &str {
        "Morph"
    }

    fn init(&mut self, ctx: &mut GameContext) {
        // load config
        self.config = Arc::new(RawConfig::new(ctx));

        // update all states
        let resource = &self.resource;
        self.states.iter_mut().for_each(|state| {
            state.init(resource);
        });
    }

    fn cleanup(&mut self, _ctx: &mut GameContext) {
        // update all states
        let resource = &self.resource;
        self.states.iter_mut().for_each(|state| {
            state.cleanup(resource);
        });
    }

    fn input(&mut self, _ctx: &mut GameContext, input_events: &[InputEvent]) {
        // update input context
        self.input.update(input_events);

        // handle input and events only of top state
        if let Some(state) = self.states.last_mut() {
            state.handle_input(&mut self.input);
        }
    }

    fn update(&mut self, ctx: &mut GameContext, elapsed_time: f32) {
        //println!("FPS: {}", 1.0 / elapsed_time);

        // update delayed events
        self.events.update_delayed(elapsed_time);

        // check state changes
        for event in self.events.read(&mut self.reader) {
            match event {
                // states
                StateEvent::Menu => {
                    info!("StateEvent: Menu");
                    self.push_state(Box::new(MenuState::new(&self.config)));
                }
                StateEvent::MenuSettings => {
                    info!("StateEvent: MenuSettings");
                    self.push_state(Box::new(MenuSettingsState::new(&self.config)));
                }
                StateEvent::MenuPackage => {
                    info!("StateEvent: MenuPackage");
                    self.push_state(Box::new(MenuPackageState::new(&self.config)));
                }
                StateEvent::MenuPackageLevel => {
                    info!("StateEvent: MenuPackageLevel");
                    if self.resource.package_info().is_some() {
                        self.push_state(Box::new(MenuPackageLevelState::new(&self.config)));
                    }
                }
                StateEvent::Level => {
                    info!("StateEvent: Level");
                    if self.resource.level_info().is_some() {
                        self.push_state(Box::new(LevelState::new(&self.config)));
                    }
                }
                StateEvent::LevelPause => {
                    info!("StateEvent: LevelPause");
                    self.push_state(Box::new(LevelPauseState::new(&self.config)));
                }
                StateEvent::LevelSuccess => {
                    info!("StateEvent: LevelSuccess");
                    self.push_state(Box::new(LevelSuccessState::new(&self.config)));
                }
                StateEvent::LevelFailure => {
                    info!("StateEvent: LevelFailure");
                    self.push_state(Box::new(LevelFailureState::new(&self.config)));
                }
                StateEvent::Back => {
                    info!("StateEvent: Back");
                    self.pop_state();
                }
                StateEvent::Exit => {
                    ctx.exit();
                }

                // content
                StateEvent::LoadPackage(package) => {
                    info!("StateEvent: LoadPackage({})", &package);
                    self.resource.load_package(ctx, &package);
                    self.graphics.load_package_textures(ctx, self.resource.package_info().unwrap());
                }
                StateEvent::UnloadPackage => {
                    info!("StateEvent: UnloadPackage");
                    self.graphics.unload_package_textures();
                    self.resource.unload_package();
                }
                StateEvent::LoadLevel(level) => {
                    info!("StateEvent: LoadLevel({})", level);
                    self.resource.load_level(level);
                }
                StateEvent::LoadLevelNext => {
                    info!("StateEvent: LoadLevelNext");
                    self.resource.load_level_next();
                }
                StateEvent::UnloadLevel => {
                    info!("StateEvent: UnloadLevel");
                    self.resource.unload_level();
                }
            }
        }

        // find all states to be updated
        let update_index = self.states.iter().rposition(|state| !state.parent_update());
        for state in (&mut self.states[update_index.unwrap_or(0)..]).iter_mut() {
            state.update(elapsed_time, &mut self.events);
        }
    }

    fn render(&mut self, _ctx: &mut GameContext, _gl: &Gl) {
        // clear frame
        self.graphics.clear();

        // find all states to be drawn
        let draw_index = self.states.iter().rposition(|state| !state.parent_draw());
        let graphics = &mut self.graphics;
        (&mut self.states[draw_index.unwrap_or(0)..]).iter_mut().for_each(|state| {
            state.draw(graphics);
        });
    }

    fn create_device(&mut self, ctx: &mut GameContext, gl: &Gl) {
        // create device context
        self.graphics.create(ctx, &self.config, gl);

        // load package graphics
        if let Some(package_info) = self.resource.package_info() {
            self.graphics.load_package_textures(ctx, package_info);
        }

        // update all states
        let graphics = &mut self.graphics;
        self.states.iter_mut().for_each(|state| {
            state.create_device(graphics);
        });
    }

    fn destroy_device(&mut self, _ctx: &mut GameContext, _gl: &Gl) {
        // update all states
        let graphics = &mut self.graphics;
        self.states.iter_mut().for_each(|state| {
            state.destroy_device(graphics);
        });

        // unload package graphics
        self.graphics.unload_package_textures();

        // destroy device context
        self.graphics.destroy();
    }

    fn resize_device(&mut self, _ctx: &mut GameContext, _gl: &Gl, width: u32, height: u32) {
        // resize device
        self.graphics.resize(width, height);

        // update input context
        self.input.change_resolution(self.graphics.resolution());

        // update all states
        let graphics = &mut self.graphics;
        self.states.iter_mut().for_each(|state| {
            state.resize_device(graphics);
        });
    }
}

impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GameManager {
    pub fn new() -> GameManager {
        let states = Vec::new();
        let mut events = Events::new();
        let reader = events.register();
        events.write(StateEvent::Menu);
        let resource = ResourceContext::new();
        let graphics = Default::default();
        let input = Default::default();
        let config = Config::default();
        GameManager {
            states,
            events,
            reader,
            resource,
            graphics,
            input,
            config,
        }
    }

    pub fn change_state(&mut self, state: Box<dyn GameState>) {
        self.pop_state();
        self.push_state(state);
    }

    pub fn push_state(&mut self, state: Box<dyn GameState>) {
        self.states.push(state);
        if let Some(state) = self.states.last_mut() {
            // init state
            state.init(&self.resource);

            // create state device
            state.create_device(&mut self.graphics);
        }
    }

    pub fn pop_state(&mut self) {
        if let Some(mut state) = self.states.pop() {
            // destroy state device
            state.destroy_device(&mut self.graphics);

            // clear state
            state.cleanup(&self.resource);
        }
    }
}
