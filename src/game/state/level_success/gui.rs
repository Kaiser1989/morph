//////////////////////////////////////////////////
// Using

use crate::game::config::*;
use crate::game::fx::*;
use crate::game::resource::gui::*;

use super::LevelSuccessEvent;

//////////////////////////////////////////////////
// GUI

pub fn create() -> GuiBuilder<LevelSuccessEvent> {
    GuiBuilder::new("success")
        .size(Value::Auto, Value::Auto)
        .texture(TEX_GUI_FADE, 0)
        .vertical()
        .children(vec![
            GuiBuilder::new("header")
                .size(Value::Auto, Value::Auto)
                .align(CENTER, CENTER)
                .text("Success", 1.5, CONFIG.color_white),
            GuiBuilder::new("buttons")
                .size(Value::Auto, Value::Auto)
                .vertical()
                .align(CENTER, CENTER)
                .children(vec![
                    GuiBuilder::new("next")
                        .size(Value::Fixed(8.0), Value::Fixed(1.25))
                        .margin(0.1, 0.1, 0.1, 0.1)
                        .texture(TEX_GUI_BAR, 0)
                        .rounded(0.2)
                        .click(LevelSuccessEvent::Next)
                        .align(CENTER, CENTER)
                        .text("Next", 0.8, CONFIG.color_white),
                    GuiBuilder::new("restart")
                        .size(Value::Fixed(8.0), Value::Fixed(1.25))
                        .margin(0.1, 0.1, 0.1, 0.1)
                        .texture(TEX_GUI_BAR, 0)
                        .rounded(0.2)
                        .click(LevelSuccessEvent::Restart)
                        .align(CENTER, CENTER)
                        .text("Restart", 0.8, CONFIG.color_white),
                    GuiBuilder::new("quit")
                        .size(Value::Fixed(8.0), Value::Fixed(1.25))
                        .margin(0.1, 0.1, 0.1, 0.1)
                        .texture(TEX_GUI_BAR, 0)
                        .rounded(0.2)
                        .click(LevelSuccessEvent::Quit)
                        .align(CENTER, CENTER)
                        .text("Quit", 0.8, CONFIG.color_white),
                ]),
        ])
}
