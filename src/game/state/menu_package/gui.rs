//////////////////////////////////////////////////
// Using

use crate::game::config::*;
use crate::game::fx::*;
use crate::game::resource::gui::*;

use super::MenuPackageEvent;

//////////////////////////////////////////////////
// GUI

pub fn create() -> GuiBuilder<MenuPackageEvent> {
    GuiBuilder::new("packages")
        .size(Value::Auto, Value::Auto)
        .padding(0.1, 0.1, 0.1, 0.1)
        .vertical()
        .align(CENTER, CENTER)
        .children(vec![
            GuiBuilder::new("top")
                .size(Value::Auto, Value::Fixed(1.5))
                .children(vec![
                    GuiBuilder::new("left")
                        .size(Value::Fixed(1.5), Value::Auto)
                        .rounded(0.2)
                        .click(MenuPackageEvent::Back)
                        .align(CENTER, CENTER)
                        .text("<", 1.2, CONFIG.color_white),
                    GuiBuilder::new("header")
                        .size(Value::Auto, Value::Auto)
                        .align(CENTER, CENTER)
                        .text("Packages", 1.0, CONFIG.color_white),
                    GuiBuilder::new("right")
                        .size(Value::Fixed(1.5), Value::Auto)
                        .align(CENTER, CENTER),
                ]),
            GuiBuilder::new("list")
                .size(Value::Auto, Value::Auto)
                .vertical()
                .align(CENTER, TOP)
                .children(
                    CONFIG
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
                                .text(package, 0.75, CONFIG.color_white)
                        })
                        .collect(),
                ),
        ])
}
