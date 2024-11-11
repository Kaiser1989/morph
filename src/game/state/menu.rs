//////////////////////////////////////////////////
// Using

use log::info;

use crate::game::config::Config;
use crate::game::game_state::{GameState, GameStateData, GameStateEvent};
use crate::game::resource::gui::*;
use crate::game::resource::{Events, GuiBuilder};
use crate::game::{fx::*, ResourceContext};
use crate::game::{StateEvent, Value};

//////////////////////////////////////////////////
// Definition

pub struct MenuState {
    data: GameStateData<MenuEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuEvent {
    Play,
    Settings,
    Exit,
}

//////////////////////////////////////////////////
// Implementation

impl GameStateEvent for MenuEvent {}

impl MenuState {
    pub fn new(config: &Config) -> Self {
        Self { data: GameStateData::new(config) }
    }
}

impl GameState for MenuState {
    type Event = MenuEvent;

    fn data(&mut self) -> &mut GameStateData<Self::Event> {
        &mut self.data
    }

    fn handle_event(&mut self, event: Self::Event, state_events: &mut Events<StateEvent>) {
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

    fn gui(&self, _resource: &ResourceContext) -> Option<GuiBuilder<Self::Event>> {
        let config = self.data.config();
        Some(GuiBuilder::new("menu").size(Value::Auto, Value::Auto).padding(0.1, 0.1, 0.1, 0.1).vertical().children(vec![
            GuiBuilder::new("top")
                .size(Value::Auto, Value::Auto)
                .align(CENTER, CENTER)
                .children(vec![GuiBuilder::new("title").size(Value::Fixed(5.0), Value::Fixed(5.0)).texture(TEX_GUI_LOGO, 0)]),
            GuiBuilder::new("bottom").size(Value::Auto, Value::Auto).vertical().align(CENTER, TOP).children(vec![
                GuiBuilder::new("packages")
                    .size(Value::Fixed(8.0), Value::Fixed(1.25))
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .rounded(0.2)
                    .click(MenuEvent::Play)
                    .align(CENTER, CENTER)
                    .text("Play", 1.0, config.color_white),
                GuiBuilder::new("settings")
                    .size(Value::Fixed(8.0), Value::Fixed(1.25))
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .rounded(0.2)
                    .click(MenuEvent::Settings)
                    .align(CENTER, CENTER)
                    .text("Settings", 1.0, config.color_white),
                GuiBuilder::new("exit")
                    .size(Value::Fixed(8.0), Value::Fixed(1.25))
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .color(1.0, 0.0, 0.0, 0.5)
                    .rounded(0.2)
                    .click(MenuEvent::Exit)
                    .align(CENTER, CENTER)
                    .text("Exit", 1.0, config.color_white),
            ]),
        ]))
    }
}
