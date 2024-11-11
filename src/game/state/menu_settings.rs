//////////////////////////////////////////////////
// Using

use log::info;

use crate::game::config::Config;
use crate::game::game_state::{GameState, GameStateData, GameStateEvent};
use crate::game::resource::gui::*;
use crate::game::resource::{Events, GuiBuilder};
use crate::game::StateEvent;
use crate::game::{fx::*, ResourceContext};

//////////////////////////////////////////////////
// Definition

pub struct MenuSettingsState {
    data: GameStateData<MenuSettingsEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuSettingsEvent {
    Back,
}

//////////////////////////////////////////////////
// Implementation

impl GameStateEvent for MenuSettingsEvent {}

impl MenuSettingsState {
    pub fn new(config: &Config) -> MenuSettingsState {
        MenuSettingsState { data: GameStateData::new(config) }
    }
}

impl GameState for MenuSettingsState {
    type Event = MenuSettingsEvent;

    fn data(&mut self) -> &mut GameStateData<Self::Event> {
        &mut self.data
    }

    fn handle_event(&mut self, event: Self::Event, state_events: &mut Events<StateEvent>) {
        match event {
            Self::Event::Back => {
                info!("MenuSettingsEvent: Back");
                state_events.write(StateEvent::Back);
            }
        }
    }

    fn gui(&self, _resource: &ResourceContext) -> Option<GuiBuilder<Self::Event>> {
        let config = self.data.config();
        Some(
            GuiBuilder::new("settings")
                .size(Value::Auto, Value::Auto)
                .padding(0.1, 0.1, 0.1, 0.1)
                .vertical()
                .align(CENTER, CENTER)
                .children(vec![
                    GuiBuilder::new("top").size(Value::Auto, Value::Fixed(1.5)).children(vec![
                        GuiBuilder::new("left")
                            .size(Value::Fixed(1.5), Value::Auto)
                            .margin(0.1, 0.1, 0.1, 0.1)
                            .texture(TEX_GUI_BAR, 0)
                            .rounded(0.2)
                            .click(MenuSettingsEvent::Back)
                            .align(CENTER, CENTER)
                            .text("<", 1.0, config.color_white),
                        GuiBuilder::new("header")
                            .size(Value::Auto, Value::Auto)
                            .margin(0.1, 0.1, 0.1, 0.1)
                            .texture(TEX_GUI_BAR, 0)
                            .rounded(0.2)
                            .align(CENTER, CENTER)
                            .text("Settings", 1.0, config.color_white),
                        GuiBuilder::new("right").size(Value::Fixed(1.5), Value::Auto).margin(0.1, 0.1, 0.1, 0.1).align(CENTER, CENTER),
                    ]),
                    GuiBuilder::new("list").size(Value::Auto, Value::Auto).vertical().align(CENTER, TOP),
                ]),
        )
    }
}
