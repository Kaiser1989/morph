//////////////////////////////////////////////////
// Using

use crate::game::config::*;
use crate::game::fx::*;
use crate::game::resource::gui::*;

use super::MenuSettingsEvent;

//////////////////////////////////////////////////
// GUI

pub fn create() -> GuiBuilder<MenuSettingsEvent> {
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
                    .text("<", 1.0, CONFIG.color_white),
                GuiBuilder::new("header")
                    .size(Value::Auto, Value::Auto)
                    .margin(0.1, 0.1, 0.1, 0.1)
                    .texture(TEX_GUI_BAR, 0)
                    .rounded(0.2)
                    .align(CENTER, CENTER)
                    .text("Settings", 1.0, CONFIG.color_white),
                GuiBuilder::new("right").size(Value::Fixed(1.5), Value::Auto).margin(0.1, 0.1, 0.1, 0.1).align(CENTER, CENTER),
            ]),
            GuiBuilder::new("list").size(Value::Auto, Value::Auto).vertical().align(CENTER, TOP),
        ])
}
