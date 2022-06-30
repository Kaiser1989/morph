//////////////////////////////////////////////////
// Using

use crate::game::config::*;
use crate::game::fx::*;
use crate::game::resource::gui::*;

use super::MenuEvent;

//////////////////////////////////////////////////
// GUI

pub fn create() -> GuiBuilder<MenuEvent> {
    GuiBuilder::new("menu").size(Value::Auto, Value::Auto).padding(0.1, 0.1, 0.1, 0.1).vertical().children(vec![
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
                .text("Play", 1.0, CONFIG.color_white),
            GuiBuilder::new("settings")
                .size(Value::Fixed(8.0), Value::Fixed(1.25))
                .margin(0.1, 0.1, 0.1, 0.1)
                .texture(TEX_GUI_BAR, 0)
                .rounded(0.2)
                .click(MenuEvent::Settings)
                .align(CENTER, CENTER)
                .text("Settings", 1.0, CONFIG.color_white),
            GuiBuilder::new("exit")
                .size(Value::Fixed(8.0), Value::Fixed(1.25))
                .margin(0.1, 0.1, 0.1, 0.1)
                .texture(TEX_GUI_BAR, 0)
                .color(1.0, 0.0, 0.0, 0.5)
                .rounded(0.2)
                .click(MenuEvent::Exit)
                .align(CENTER, CENTER)
                .text("Exit", 1.0, CONFIG.color_white),
        ]),
    ])
}
