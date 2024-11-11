//////////////////////////////////////////////////
// Using

use log::info;

use crate::game::config::Config;
use crate::game::game_state::{GameStateEvent, GameState, GameStateData};
use crate::game::resource::gui::*;
use crate::game::resource::{Events, GuiBuilder};
use crate::game::{fx::*, ResourceContext};
use crate::game::{StateEvent, Value};

//////////////////////////////////////////////////
// Definition

pub struct MenuPackageLevelState {
    data: GameStateData<MenuPackageLevelEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuPackageLevelEvent {
    Level(usize),
    Back,
}

//////////////////////////////////////////////////
// Implementation

impl GameStateEvent for MenuPackageLevelEvent {}

impl MenuPackageLevelState {
    pub fn new(config: &Config) -> Self {
        Self { data: GameStateData::new(config) }
    }
}

impl GameState for MenuPackageLevelState {
    type Event = MenuPackageLevelEvent;

    fn data(&mut self) -> &mut GameStateData<Self::Event> {
        &mut self.data
    }

    fn handle_event(&mut self, event: Self::Event, state_events: &mut Events<StateEvent>) {
        match event {
            MenuPackageLevelEvent::Level(level) => {
                info!("MenuPackageLevelEvent: Level({})", level);
                state_events.write(StateEvent::LoadLevel(level));
                state_events.write(StateEvent::Level);
            }
            MenuPackageLevelEvent::Back => {
                info!("MenuPackageLevelEvent: Back");
                state_events.write(StateEvent::UnloadLevel);
                state_events.write(StateEvent::Back);
            }
        }
    }

    fn gui(&self, _resource: &ResourceContext) -> Option<GuiBuilder<Self::Event>> {
        let config = self.data.config();
        Some(
            GuiBuilder::new("package")
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
                            .click(MenuPackageLevelEvent::Back)
                            .align(CENTER, CENTER)
                            .text("<", 1.0, config.color_white),
                        GuiBuilder::new("header")
                            .size(Value::Auto, Value::Auto)
                            .margin(0.1, 0.1, 0.1, 0.1)
                            .texture(TEX_GUI_BAR, 0)
                            .rounded(0.2)
                            .align(CENTER, CENTER)
                            .text("Level List", 1.0, config.color_white),
                        GuiBuilder::new("right").size(Value::Fixed(1.5), Value::Auto).margin(0.1, 0.1, 0.1, 0.1).align(CENTER, CENTER),
                    ]),
                    GuiBuilder::new("levels").size(Value::Auto, Value::Auto).vertical().align(CENTER, TOP).children(
                        (0..4)
                            .map(|r| {
                                GuiBuilder::new("row").size(Value::Auto, Value::Auto).children(
                                    (0..5)
                                        .map(|c| {
                                            let index = r * 5 + c;
                                            GuiBuilder::new("level")
                                                .size(Value::Auto, Value::Auto)
                                                .margin(0.1, 0.1, 0.1, 0.1)
                                                .padding(0.1, 0.1, 0.1, 0.1)
                                                .texture(TEX_GUI_BAR, 0)
                                                .rounded(0.2)
                                                .click(MenuPackageLevelEvent::Level(index))
                                                .align(CENTER, TOP)
                                                .text(&format!("{}", index + 1), 0.75, config.color_white)
                                        })
                                        .collect(),
                                )
                            })
                            .collect(),
                    ),
                ]),
        )
    }
}
