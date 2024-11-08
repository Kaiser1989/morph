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

pub struct LevelFailureState {
    data: GuiStateData<LevelFailureEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LevelFailureEvent {
    Restart,
    Quit,
}

//////////////////////////////////////////////////
// Implementation

impl GameStateEvent for LevelFailureEvent {}

impl LevelFailureState {
    pub fn new(config: &Config) -> Self {
        Self { data: GuiStateData::new(config) }
    }
}

impl GuiState for LevelFailureState {
    type Event = LevelFailureEvent;

    fn data(&mut self) -> &mut GuiStateData<Self::Event> {
        &mut self.data
    }

    fn handle_event(&mut self, event: Self::Event, state_events: &mut Events<StateEvent>) {
        match event {
            LevelFailureEvent::Restart => {
                info!("LevelFailureEvent: Restart");
                state_events.write(StateEvent::Back);
                state_events.write(StateEvent::Back);
                state_events.write(StateEvent::Level);
            }
            LevelFailureEvent::Quit => {
                info!("LevelFailureEvent: Quit");
                state_events.write(StateEvent::UnloadLevel);
                state_events.write(StateEvent::Back);
                state_events.write(StateEvent::Back);
            }
        }
    }

    fn gui(&self, _resource: &ResourceContext) -> Option<GuiBuilder<Self::Event>> {
        let config = self.data.config();
        Some(GuiBuilder::new("failure").size(Value::Auto, Value::Auto).texture(TEX_GUI_FADE, 0).vertical().children(vec![
            GuiBuilder::new("header").size(Value::Auto, Value::Auto).align(CENTER, CENTER).text("Failure", 1.5, config.color_white),
            GuiBuilder::new("buttons").size(Value::Auto, Value::Auto).vertical().align(CENTER, CENTER).children(vec![
                GuiBuilder::new("restart")
                    .size(Value::Fixed(8.0), Value::Fixed(1.25))
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .rounded(0.2)
                    .click(LevelFailureEvent::Restart)
                    .align(CENTER, CENTER)
                    .text("Restart", 0.8, config.color_white),
                GuiBuilder::new("quit")
                    .size(Value::Fixed(8.0), Value::Fixed(1.25))
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .rounded(0.2)
                    .click(LevelFailureEvent::Quit)
                    .align(CENTER, CENTER)
                    .text("Quit", 0.8, config.color_white),
            ]),
        ]))
    }
}
