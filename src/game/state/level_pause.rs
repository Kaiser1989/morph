//////////////////////////////////////////////////
// Using

use log::info;

use crate::game::config::Config;
use crate::game::game_state::{GameStateEvent, GameState, GameStateData};
use crate::game::resource::gui::*;
use crate::game::resource::{Events, GuiBuilder};
use crate::game::StateEvent;
use crate::game::{fx::*, ResourceContext};

//////////////////////////////////////////////////
// Definition

pub struct LevelPauseState {
    data: GameStateData<LevelPauseEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LevelPauseEvent {
    Resume,
    Restart,
    Quit,
}

//////////////////////////////////////////////////
// Implementation

impl GameStateEvent for LevelPauseEvent {}

impl LevelPauseState {
    pub fn new(config: &Config) -> LevelPauseState {
        Self { data: GameStateData::new(config) }
    }
}

impl GameState for LevelPauseState {
    type Event = LevelPauseEvent;

    fn data(&mut self) -> &mut GameStateData<Self::Event> {
        &mut self.data
    }

    fn handle_event(&mut self, event: Self::Event, state_events: &mut Events<StateEvent>) {
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

    fn gui(&self, _resource: &ResourceContext) -> Option<GuiBuilder<Self::Event>> {
        let config = self.data.config();
        Some(GuiBuilder::new("pause").size(Value::Auto, Value::Auto).texture(TEX_GUI_FADE, 0).vertical().children(vec![
            GuiBuilder::new("header").size(Value::Auto, Value::Auto).align(CENTER, CENTER).text("Pause", 1.5, config.color_white),
            GuiBuilder::new("buttons").size(Value::Auto, Value::Auto).vertical().align(CENTER, CENTER).children(vec![
                GuiBuilder::new("resume")
                    .click(LevelPauseEvent::Resume)
                    .size(Value::Fixed(8.0), Value::Fixed(1.25))
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .rounded(0.2)
                    .align(CENTER, CENTER)
                    .text("Resume", 0.8, config.color_white),
                GuiBuilder::new("restart")
                    .click(LevelPauseEvent::Restart)
                    .size(Value::Fixed(8.0), Value::Fixed(1.25))
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .rounded(0.2)
                    .align(CENTER, CENTER)
                    .text("Restart", 0.8, config.color_white),
                GuiBuilder::new("quit")
                    .click(LevelPauseEvent::Quit)
                    .size(Value::Fixed(8.0), Value::Fixed(1.25))
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .rounded(0.2)
                    .align(CENTER, CENTER)
                    .text("Quit", 0.8, config.color_white),
            ]),
        ]))
    }
}
