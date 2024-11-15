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
use game_state::InternalGameState;
use log::info;
use shrev::ReaderId;

use crate::game::fx::*;
use crate::game::resource::*;
use crate::game::state::*;

//////////////////////////////////////////////////
// Definition

pub struct GameApplicationData {
    states: Vec<Box<dyn InternalGameState>>,
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

pub trait GameApplication: Default {
    fn data(&mut self) -> &mut GameApplicationData;

    fn title(&self) -> &str;

    fn start_event(&self) -> StateEvent;

    fn handle_events(&mut self, event: StateEvent, events: &mut Events<StateEvent>);
}

impl<A> GameLoop for A
where
    A: GameApplication,
{
    fn title(&self) -> &str {
        "Morph"
    }

    fn init(&mut self, ctx: &mut GameContext) {
        let data = self.data();

        // load config
        data.config = Arc::new(RawConfig::new(ctx));

        // update all states
        data.states.iter_mut().for_each(|state| {
            state.init(&data.resource);
        });
    }

    fn cleanup(&mut self, _ctx: &mut GameContext) {
        let data = self.data();

        // update all states
        data.states.iter_mut().for_each(|state| {
            state.cleanup(&data.resource);
        });
    }

    fn input(&mut self, _ctx: &mut GameContext, input_events: &[InputEvent]) {
        let data = self.data();

        // update input context
        data.input.update(input_events);

        // handle input and events only of top state
        if let Some(state) = data.states.last_mut() {
            state.handle_input(&mut data.input);
        }
    }

    fn update(&mut self, ctx: &mut GameContext, elapsed_time: f32) {
        let data = self.data();
        //println!("FPS: {}", 1.0 / elapsed_time);

        // update delayed events
        data.events.update_delayed(elapsed_time);

        // check state changes
        for event in data.events.read(&mut data.reader) {
            match event {
                // states
                StateEvent::Menu => {
                    info!("StateEvent: Menu");
                    self.push_state(Box::new(MenuState::new(&data.config)));
                }
                StateEvent::MenuSettings => {
                    info!("StateEvent: MenuSettings");
                    self.push_state(Box::new(MenuSettingsState::new(&data.config)));
                }
                StateEvent::MenuPackage => {
                    info!("StateEvent: MenuPackage");
                    self.push_state(Box::new(MenuPackageState::new(&data.config)));
                }
                StateEvent::MenuPackageLevel => {
                    info!("StateEvent: MenuPackageLevel");
                    if self.resource.package_info().is_some() {
                        self.push_state(Box::new(MenuPackageLevelState::new(&data.config)));
                    }
                }
                StateEvent::Level => {
                    info!("StateEvent: Level");
                    if self.resource.level_info().is_some() {
                        self.push_state(Box::new(LevelState::new(&data.config)));
                    }
                }
                StateEvent::LevelPause => {
                    info!("StateEvent: LevelPause");
                    self.push_state(Box::new(LevelPauseState::new(&data.config)));
                }
                StateEvent::LevelSuccess => {
                    info!("StateEvent: LevelSuccess");
                    self.push_state(Box::new(LevelSuccessState::new(&data.config)));
                }
                StateEvent::LevelFailure => {
                    info!("StateEvent: LevelFailure");
                    self.push_state(Box::new(LevelFailureState::new(&data.config)));
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
                    data.resource.load_package(ctx, &package);
                    data.graphics.lock().unwrap().load_package_textures(ctx, data.resource.package_info().unwrap());
                }
                StateEvent::UnloadPackage => {
                    info!("StateEvent: UnloadPackage");
                    data.graphics.lock().unwrap().unload_package_textures();
                    data.resource.unload_package();
                }
                StateEvent::LoadLevel(level) => {
                    info!("StateEvent: LoadLevel({})", level);
                    data.resource.load_level(level);
                }
                StateEvent::LoadLevelNext => {
                    info!("StateEvent: LoadLevelNext");
                    data.resource.load_level_next();
                }
                StateEvent::UnloadLevel => {
                    info!("StateEvent: UnloadLevel");
                    data.resource.unload_level();
                }
            }
        }

        // find all states to be updated
        let update_index = data.states.iter().rposition(|state| !state.parent_update());
        for state in (&mut data.states[update_index.unwrap_or(0)..]).iter_mut() {
            state.update(elapsed_time, &mut data.events);
        }
    }

    fn render(&mut self, _ctx: &mut GameContext, _gl: &Gl) {
        let data = self.data();

        // clear frame
        data.graphics.lock().unwrap().clear();

        // find all states to be drawn
        let draw_index = data.states.iter().rposition(|state| !state.parent_draw());
        (&mut data.states[draw_index.unwrap_or(0)..]).iter_mut().for_each(|state| {
            state.draw(&data.graphics);
        });
    }

    fn create_device(&mut self, ctx: &mut GameContext, gl: &Gl) {
        let data = self.data();

        // create device context
        data.graphics.lock().unwrap().create(ctx, &data.config, gl);

        // load package graphics
        if let Some(package_info) = data.resource.package_info() {
            data.graphics.lock().unwrap().load_package_textures(ctx, package_info);
        }

        // update all states
        data.states.iter_mut().for_each(|state| {
            state.create_device(&data.graphics);
        });
    }

    fn destroy_device(&mut self, _ctx: &mut GameContext, _gl: &Gl) {
        let data = self.data();

        // update all states
        data.states.iter_mut().for_each(|state| {
            state.destroy_device(&data.graphics);
        });

        // unload package graphics
        data.graphics.lock().unwrap().unload_package_textures();

        // destroy device context
        data.graphics.lock().unwrap().destroy();
    }

    fn resize_device(&mut self, _ctx: &mut GameContext, _gl: &Gl, width: u32, height: u32) {
        let data = self.data();

        // resize device
        data.graphics.lock().unwrap().resize(width, height);

        // update input context
        data.input.change_resolution(data.graphics.lock().unwrap().resolution());

        // update all states
        let graphics = &mut data.graphics;
        data.states.iter_mut().for_each(|state| {
            state.resize_device(graphics);
        });
    }
}

impl Default for GameApplicationData {
    fn default() -> Self {
        Self::new()
    }
}

impl GameApplicationData {
    pub fn new() -> Self {
        let states = Vec::new();
        let mut events = Events::new();
        let reader = events.register();
        events.write(StateEvent::Menu);
        let resource = ResourceContext::new();
        let graphics = Default::default();
        let input = Default::default();
        let config = Config::default();
        Self {
            states,
            events,
            reader,
            resource,
            graphics,
            input,
            config,
        }
    }

    pub fn change_state(&mut self, state: Box<dyn InternalGameState>) {
        self.pop_state();
        self.push_state(state);
    }

    pub fn push_state(&mut self, state: Box<dyn InternalGameState>) {
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
