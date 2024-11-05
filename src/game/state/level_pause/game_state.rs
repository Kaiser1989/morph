//////////////////////////////////////////////////
// Using

use log::info;
use nalgebra_glm::*;
use shrev::ReaderId;

use crate::game::config::Config;
use crate::game::fx::GraphicsContext;
use crate::game::resource::{Events, Gui, GuiBuilder, InputContext, ResourceContext};
use crate::game::state::GameState;
use crate::game::StateEvent;

use super::gui;

//////////////////////////////////////////////////
// Definition

pub struct LevelPauseState {
    config: Config,
    gui: Gui<LevelPauseEvent>,
    events: Events<LevelPauseEvent>,
    reader: ReaderId<LevelPauseEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LevelPauseEvent {
    Resume,
    Restart,
    Quit,
}

//////////////////////////////////////////////////
// Implementation

impl LevelPauseState {
    pub fn new(config: &Config) -> LevelPauseState {
        let config = config.clone();
        let gui = Gui::new(&config);
        let mut events = Events::new();
        let reader = events.register();
        LevelPauseState { config, gui, events, reader }
    }
}

impl GameState for LevelPauseState {
    fn init(&mut self, _resource: &ResourceContext) {
        // init gui
        self.gui.init(&gui::create(&self.config));
    }

    fn cleanup(&mut self, _resource: &ResourceContext) {
        // clear gui
        self.gui.init(&GuiBuilder::new(""));
    }

    fn handle_input(&mut self, input: &InputContext) {
        // handle back button
        if input.back() {
            self.events.write(LevelPauseEvent::Quit);
        }

        // handle gui click
        self.gui.handle_input(input, &mut self.events);
    }

    fn update(&mut self, elapsed_time: f32, state_events: &mut Events<StateEvent>) {
        // update delayed events
        self.events.update_delayed(elapsed_time);

        // handle events
        for event in self.events.read(&mut self.reader) {
            match event {
                LevelPauseEvent::Resume => {
                    info!("LevelPauseEvent: Resume");
                    state_events.write(StateEvent::Back);
                }
                LevelPauseEvent::Restart => {
                    info!("LevelPauseEvent: Restart");
                    state_events.write(StateEvent::Back);
                    state_events.write(StateEvent::Back);
                    state_events.write(StateEvent::Level);
                }
                LevelPauseEvent::Quit => {
                    info!("LevelPauseEvent: Quit");
                    state_events.write(StateEvent::UnloadLevel);
                    state_events.write(StateEvent::Back);
                    state_events.write(StateEvent::Back);
                }
            }
        }
    }

    fn draw(&mut self, graphics: &mut GraphicsContext) {
        // draw gui
        self.gui.draw(graphics);
    }

    fn create_device(&mut self, graphics: &mut GraphicsContext) {
        // adjust gui dimension
        self.gui.adjust_dimension(graphics.resolution());
    }

    fn resize_device(&mut self, graphics: &mut GraphicsContext) {
        // adjust gui dimension
        self.gui.adjust_dimension(graphics.resolution());
    }

    fn destroy_device(&mut self, _graphics: &mut GraphicsContext) {
        // adjust gui dimension
        self.gui.adjust_dimension(Vec2::zeros());
    }

    fn parent_draw(&self) -> bool {
        true
    }
}
