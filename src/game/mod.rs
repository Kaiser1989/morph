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

use game_gl::prelude::*;
use log::info;
use shrev::ReaderId;
use simple_logger::SimpleLogger;

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
// Entry Point

pub fn start() {
    // init logging
    #[cfg(debug_assertions)]
    let log_level = log::LevelFilter::Info;
    #[cfg(not(debug_assertions))]
    let log_level = log::LevelFilter::Info;
    match SimpleLogger::new().with_utc_timestamps().with_level(log_level).init() {
        Err(s) => println!("{}", s),
        _ => {}
    }

    // init game loop and run
    let mut game_loop = GameLoop::new(GameManager::new());
    game_loop.run();
}

//////////////////////////////////////////////////
// Implementation

impl Runner for GameManager {
    fn init(&mut self) {
        // update all states
        let resource = &self.resource;
        self.states.iter_mut().for_each(|state| {
            state.init(resource);
        });
    }

    fn cleanup(&mut self) {
        // update all states
        let resource = &self.resource;
        self.states.iter_mut().for_each(|state| {
            state.cleanup(resource);
        });
    }

    fn input(&mut self, input_events: &[InputEvent]) {
        // update input context
        self.input.update(input_events);

        // handle input and events only of top state
        if let Some(state) = self.states.last_mut() {
            state.handle_input(&mut self.input);
        }
    }

    fn update(&mut self, elapsed_time: f32) {
        //println!("FPS: {}", 1.0 / elapsed_time);

        // update delayed events
        self.events.update_delayed(elapsed_time);

        // check state changes
        for event in self.events.read(&mut self.reader) {
            match event {
                // states
                StateEvent::Menu => {
                    info!("StateEvent: Menu");
                    self.push_state(Box::new(MenuState::new()));
                }
                StateEvent::MenuSettings => {
                    info!("StateEvent: MenuSettings");
                    self.push_state(Box::new(MenuSettingsState::new()));
                }
                StateEvent::MenuPackage => {
                    info!("StateEvent: MenuPackage");
                    self.push_state(Box::new(MenuPackageState::new()));
                }
                StateEvent::MenuPackageLevel => {
                    info!("StateEvent: MenuPackageLevel");
                    if self.resource.package_info().is_some() {
                        self.push_state(Box::new(MenuPackageLevelState::new()));
                    }
                }
                StateEvent::Level => {
                    info!("StateEvent: Level");
                    if self.resource.level_info().is_some() {
                        self.push_state(Box::new(LevelState::new()));
                    }
                }
                StateEvent::LevelPause => {
                    info!("StateEvent: LevelPause");
                    self.push_state(Box::new(LevelPauseState::new()));
                }
                StateEvent::LevelSuccess => {
                    info!("StateEvent: LevelSuccess");
                    self.push_state(Box::new(LevelSuccessState::new()));
                }
                StateEvent::LevelFailure => {
                    info!("StateEvent: LevelFailure");
                    self.push_state(Box::new(LevelFailureState::new()));
                }
                StateEvent::Back => {
                    info!("StateEvent: Back");
                    self.pop_state();
                }
                StateEvent::Exit => {
                    info!("StateEvent: Exit");
                    #[cfg(target_os = "android")]
                    ndk_glue::native_activity().finish();
                    #[cfg(not(target_os = "android"))]
                    std::process::exit(0);
                }

                // content
                StateEvent::LoadPackage(package) => {
                    info!("StateEvent: LoadPackage({})", &package);
                    self.resource.load_package(&package);
                    self.graphics.load_package_textures(self.resource.package_info().unwrap());
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

    fn render(&mut self, _gl: &Gl) {
        // clear frame
        self.graphics.clear();

        // find all states to be drawn
        let draw_index = self.states.iter().rposition(|state| !state.parent_draw());
        let graphics = &mut self.graphics;
        (&mut self.states[draw_index.unwrap_or(0)..]).iter_mut().for_each(|state| {
            state.draw(graphics);
        });
    }

    fn create_device(&mut self, gl: &Gl) {
        // create device context
        self.graphics.create(gl);

        // load package graphics
        if let Some(package_info) = self.resource.package_info() {
            self.graphics.load_package_textures(package_info);
        }

        // update all states
        let graphics = &mut self.graphics;
        self.states.iter_mut().for_each(|state| {
            state.create_device(graphics);
        });
    }

    fn destroy_device(&mut self, _gl: &Gl) {
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

    fn resize_device(&mut self, _gl: &Gl, width: u32, height: u32) {
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

impl GameManager {
    pub fn new() -> GameManager {
        let states = Vec::new();
        let mut events = Events::new();
        let reader = events.register();
        events.write(StateEvent::Menu);
        let resource = ResourceContext::new();
        let graphics = Default::default();
        let input = Default::default();
        GameManager {
            states,
            events,
            reader,
            resource,
            graphics,
            input,
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
