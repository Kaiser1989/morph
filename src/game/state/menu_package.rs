//////////////////////////////////////////////////
// Using

use log::info;

use crate::game::config::Config;
use crate::game::game_state::{GameStateEvent, GuiState, GuiStateData};
use crate::game::resource::gui::*;
use crate::game::resource::Events;
use crate::game::{fx::*, ResourceContext};
use crate::game::{GuiBuilder, StateEvent};

//////////////////////////////////////////////////
// Definition

pub struct MenuPackageState {
    data: GuiStateData<MenuPackageEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MenuPackageEvent {
    Package(String),
    Back,
}

//////////////////////////////////////////////////
// Implementation

impl GameStateEvent for MenuPackageEvent {}

impl MenuPackageState {
    pub fn new(config: &Config) -> Self {
        Self { data: GuiStateData::new(config) }
    }
}

impl GuiState for MenuPackageState {
    type Event = MenuPackageEvent;

    fn data(&mut self) -> &mut GuiStateData<Self::Event> {
        &mut self.data
    }

    fn handle_event(&mut self, event: Self::Event, state_events: &mut Events<StateEvent>) {
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

    fn gui(&self, _resource: &ResourceContext) -> Option<GuiBuilder<Self::Event>> {
        let config = self.data.config();
        Some(
            GuiBuilder::new("packages")
                .size(Value::Auto, Value::Auto)
                .padding(0.1, 0.1, 0.1, 0.1)
                .vertical()
                .align(CENTER, CENTER)
                .children(vec![
                    GuiBuilder::new("top").size(Value::Auto, Value::Fixed(1.5)).children(vec![
                        GuiBuilder::new("left")
                            .size(Value::Fixed(1.5), Value::Auto)
                            .rounded(0.2)
                            .click(MenuPackageEvent::Back)
                            .align(CENTER, CENTER)
                            .text("<", 1.2, config.color_white),
                        GuiBuilder::new("header").size(Value::Auto, Value::Auto).align(CENTER, CENTER).text("Packages", 1.0, config.color_white),
                        GuiBuilder::new("right").size(Value::Fixed(1.5), Value::Auto).align(CENTER, CENTER),
                    ]),
                    GuiBuilder::new("list").size(Value::Auto, Value::Auto).vertical().align(CENTER, TOP).children(
                        config
                            .packages
                            .iter()
                            .map(|package| {
                                GuiBuilder::new("package")
                                    .size(Value::Auto, Value::Auto)
                                    .margin(0.1, 0.1, 0.1, 0.1)
                                    .padding(0.1, 0.1, 0.1, 0.1)
                                    .texture(TEX_GUI_BAR, 0)
                                    .rounded(0.2)
                                    .click(MenuPackageEvent::Package(package.clone()))
                                    .align(CENTER, TOP)
                                    .text(package, 0.75, config.color_white)
                            })
                            .collect(),
                    ),
                ]),
        )
    }
}
