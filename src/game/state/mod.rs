//////////////////////////////////////////////////
// Modules

pub mod menu;
pub(crate) use menu::MenuState;

pub mod menu_package;
pub(crate) use menu_package::MenuPackageState;

pub mod menu_package_level;
pub(crate) use menu_package_level::MenuPackageLevelState;

pub mod menu_settings;
pub(crate) use menu_settings::MenuSettingsState;

pub mod level;
pub(crate) use level::LevelState;

pub mod level_pause;
pub(crate) use level_pause::LevelPauseState;

pub mod level_success;
pub(crate) use level_success::LevelSuccessState;

pub mod level_failure;
pub(crate) use level_failure::LevelFailureState;

//////////////////////////////////////////////////
// Using

use crate::game::fx::GraphicsContext;
use crate::game::resource::{Events, InputContext, ResourceContext};
use crate::game::StateEvent;

//////////////////////////////////////////////////
// Trait

pub trait GameState {
    fn init(&mut self, resource: &ResourceContext);

    fn cleanup(&mut self, resource: &ResourceContext);

    fn handle_input(&mut self, input: &InputContext);

    fn update(&mut self, elapsed_time: f32, events: &mut Events<StateEvent>);

    fn draw(&mut self, graphics: &mut GraphicsContext);

    fn create_device(&mut self, graphics: &mut GraphicsContext);

    fn destroy_device(&mut self, graphics: &mut GraphicsContext);

    fn resize_device(&mut self, graphics: &mut GraphicsContext);

    fn parent_update(&self) -> bool {
        false
    }

    fn parent_draw(&self) -> bool {
        false
    }
}
