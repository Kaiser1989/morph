//////////////////////////////////////////////////
// Using

use crate::game::config::Config;
use crate::game::fx::*;
use crate::game::resource::gui::*;

use super::MenuPackageLevelEvent;

//////////////////////////////////////////////////
// GUI

pub fn create(config: &Config) -> GuiBuilder<MenuPackageLevelEvent> {
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
        ])
}
