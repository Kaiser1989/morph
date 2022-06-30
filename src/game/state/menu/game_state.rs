//////////////////////////////////////////////////
// Using

use log::info;
use nalgebra_glm::*;
use shrev::ReaderId;

use crate::game::fx::GraphicsContext;
use crate::game::resource::{Events, Gui, GuiBuilder, InputContext, ResourceContext};
use crate::game::state::GameState;
use crate::game::StateEvent;

use super::gui;

//////////////////////////////////////////////////
// Definition

pub struct MenuState {
    gui: Gui<MenuEvent>,
    events: Events<MenuEvent>,
    reader: ReaderId<MenuEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuEvent {
    Play,
    Settings,
    Exit,
}

//////////////////////////////////////////////////
// Implementation

impl MenuState {
    pub fn new() -> MenuState {
        let gui = Gui::new();
        let mut events = Events::new();
        let reader = events.register();
        MenuState {
            gui,
            events,
            reader,
        }
    }
}

impl GameState for MenuState {
    fn init(&mut self, _resource: &ResourceContext) {
        // init gui
        self.gui.init(&gui::create());
    }

    fn cleanup(&mut self, _resource: &ResourceContext) {
        // clear gui
        self.gui.init(&GuiBuilder::new(""));
    }

    fn handle_input(&mut self, input: &InputContext) {
        // handle back button
        if input.back() {
            self.events.write(MenuEvent::Exit);
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
                MenuEvent::Play => {
                    info!("MenuEvent: Play");
                    state_events.write(StateEvent::MenuPackage);
                }
                MenuEvent::Settings => {
                    info!("MenuEvent: Settings");
                    state_events.write(StateEvent::MenuSettings);
                }
                MenuEvent::Exit => {
                    info!("MenuEvent: Exit");
                    state_events.write(StateEvent::Exit);
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
}
