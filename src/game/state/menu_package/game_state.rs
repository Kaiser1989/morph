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

pub struct MenuPackageState {
    gui: Gui<MenuPackageEvent>,
    events: Events<MenuPackageEvent>,
    reader: ReaderId<MenuPackageEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuPackageEvent {
    Package(String),
    Back,
}

//////////////////////////////////////////////////
// Implementation

impl MenuPackageState {
    pub fn new() -> MenuPackageState {
        let gui = Gui::new();
        let mut events = Events::new();
        let reader = events.register();
        MenuPackageState {
            gui,
            events,
            reader,
        }
    }
}

impl GameState for MenuPackageState {
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
            self.events.write(MenuPackageEvent::Back);
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
                MenuPackageEvent::Package(package) => {
                    info!("MenuPackageEvent: Package({})", package);
                    state_events.write(StateEvent::LoadPackage(package));
                    state_events.write(StateEvent::MenuPackageLevel);
                }
                MenuPackageEvent::Back => {
                    info!("MenuPackageEvent: Back");
                    state_events.write(StateEvent::UnloadPackage);
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
}
