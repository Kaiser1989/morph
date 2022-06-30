//////////////////////////////////////////////////
// Using

use crate::game::config::*;
use crate::game::fx::*;
use crate::game::resource::gui::*;

use super::LevelPauseEvent;

//////////////////////////////////////////////////
// GUI

pub fn create() -> GuiBuilder<LevelPauseEvent> {
    GuiBuilder::new("pause")
        .size(Value::Auto, Value::Auto)
        .texture(TEX_GUI_FADE, 0)
        .vertical()
        .children(vec![
            GuiBuilder::new("header")
                .size(Value::Auto, Value::Auto)
                .align(CENTER, CENTER)
                .text("Pause", 1.5, CONFIG.color_white),
            GuiBuilder::new("buttons")
                .size(Value::Auto, Value::Auto)
                .vertical()
                .align(CENTER, CENTER)
                .children(vec![
                    GuiBuilder::new("resume")
                        .click(LevelPauseEvent::Resume)
                        .size(Value::Fixed(8.0), Value::Fixed(1.25))
                        .margin(0.1, 0.1, 0.1, 0.1)
                        .texture(TEX_GUI_BAR, 0)
                        .rounded(0.2)
                        .align(CENTER, CENTER)
                        .text("Resume", 0.8, CONFIG.color_white),
                    GuiBuilder::new("restart")
                        .click(LevelPauseEvent::Restart)
                        .size(Value::Fixed(8.0), Value::Fixed(1.25))
                        .margin(0.1, 0.1, 0.1, 0.1)
                        .texture(TEX_GUI_BAR, 0)
                        .rounded(0.2)
                        .align(CENTER, CENTER)
                        .text("Restart", 0.8, CONFIG.color_white),
                    GuiBuilder::new("quit")
                        .click(LevelPauseEvent::Quit)
                        .size(Value::Fixed(8.0), Value::Fixed(1.25))
                        .margin(0.1, 0.1, 0.1, 0.1)
                        .texture(TEX_GUI_BAR, 0)
                        .rounded(0.2)
                        .align(CENTER, CENTER)
                        .text("Quit", 0.8, CONFIG.color_white),
                ]),
        ])
}
