//////////////////////////////////////////////////
// Using

use log::info;

use crate::game::config::Config;
use crate::game::game_state::{GameStateEvent, GuiState, GuiStateData};
use crate::game::resource::gui::*;
use crate::game::resource::{Events, GuiBuilder};
use crate::game::StateEvent;
use crate::game::{fx::*, ResourceContext};

//////////////////////////////////////////////////
// Definition

pub struct LevelSuccessState {
    data: GuiStateData<LevelSuccessEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LevelSuccessEvent {
    Next,
    Restart,
    Quit,
}

//////////////////////////////////////////////////
// Implementation

impl GameStateEvent for LevelSuccessEvent {}

impl LevelSuccessState {
    pub fn new(config: &Config) -> Self {
        Self { data: GuiStateData::new(config) }
    }
}

impl GuiState for LevelSuccessState {
    type Event = LevelSuccessEvent;

    fn data(&mut self) -> &mut GuiStateData<Self::Event> {
        &mut self.data
    }

    fn handle_event(&mut self, event: Self::Event, state_events: &mut Events<StateEvent>) {
        match event {
            LevelSuccessEvent::Next => {
                info!("LevelSuccessEvent: Next");
                state_events.write(StateEvent::LoadLevelNext);
                state_events.write(StateEvent::Back);
                state_events.write(StateEvent::Back);
                state_events.write(StateEvent::Level);
            }
            LevelSuccessEvent::Restart => {
                info!("LevelSuccessEvent: Restart");
                state_events.write(StateEvent::Back);
                state_events.write(StateEvent::Back);
                state_events.write(StateEvent::Level);
            }
            LevelSuccessEvent::Quit => {
                info!("LevelSuccessEvent: Quit");
                state_events.write(StateEvent::UnloadLevel);
                state_events.write(StateEvent::Back);
                state_events.write(StateEvent::Back);
            }
        }
    }

    fn gui(&self, _resource: &ResourceContext) -> Option<GuiBuilder<Self::Event>> {
        let config = self.data.config();
        Some(GuiBuilder::new("success").size(Value::Auto, Value::Auto).texture(TEX_GUI_FADE, 0).vertical().children(vec![
            GuiBuilder::new("header").size(Value::Auto, Value::Auto).align(CENTER, CENTER).text("Success", 1.5, config.color_white),
            GuiBuilder::new("buttons").size(Value::Auto, Value::Auto).vertical().align(CENTER, CENTER).children(vec![
                GuiBuilder::new("next")
                    .size(Value::Fixed(8.0), Value::Fixed(1.25))
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .rounded(0.2)
                    .click(LevelSuccessEvent::Next)
                    .align(CENTER, CENTER)
                    .text("Next", 0.8, config.color_white),
                GuiBuilder::new("restart")
                    .size(Value::Fixed(8.0), Value::Fixed(1.25))
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .rounded(0.2)
                    .click(LevelSuccessEvent::Restart)
                    .align(CENTER, CENTER)
                    .text("Restart", 0.8, config.color_white),
                GuiBuilder::new("quit")
                    .size(Value::Fixed(8.0), Value::Fixed(1.25))
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .rounded(0.2)
                    .click(LevelSuccessEvent::Quit)
                    .align(CENTER, CENTER)
                    .text("Quit", 0.8, config.color_white),
            ]),
        ]))
    }
}
