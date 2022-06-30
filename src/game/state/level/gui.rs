//////////////////////////////////////////////////
// Using

use crate::game::config::*;
use crate::game::ecs::resource::MorphState;
use crate::game::fx::*;
use crate::game::resource::gui::*;
use crate::game::resource::ResourceContext;

use super::{LevelEvent, LevelPhase};

//////////////////////////////////////////////////
// GUI

pub fn create(resource: &ResourceContext, phase: LevelPhase) -> GuiBuilder<LevelEvent> {
    if let (Some(package_info), Some(level), Some(level_info)) = (
        resource.package_info(),
        resource.level(),
        resource.level_info(),
    ) {
        match phase {
            LevelPhase::Preview => GuiBuilder::new("preview")
                .size(Value::Auto, Value::Auto)
                .vertical()
                .click(LevelEvent::Start)
                .align(CENTER, CENTER)
                .children(vec![
                    GuiBuilder::new("top")
                        .size(Value::Auto, Value::Fixed(2.0))
                        .margin(0.0, 0.0, 0.0, 0.5)
                        .vertical()
                        .align(CENTER, TOP)
                        .children(vec![
                            GuiBuilder::new("title")
                                .size(Value::Auto, Value::Auto)
                                .align(CENTER, CENTER)
                                .text(
                                    &format!("{}: #{}", package_info.name, level),
                                    0.8,
                                    CONFIG.color_white,
                                ),
                            GuiBuilder::new("morphs")
                                .size(Value::Auto, Value::Auto)
                                .align(CENTER, CENTER)
                                .children(vec![
                                    GuiBuilder::new(MorphState::Metal.to_string())
                                        .size(Value::Fixed(0.8), Value::Fixed(0.8))
                                        .margin(0.05, 0.05, 0.0, 0.0)
                                        .texture(TEX_GUI_METAL, 0)
                                        .align(CENTER, CENTER)
                                        .text(
                                            &format!(
                                                "{}",
                                                level_info.available_morphs[MorphState::Metal]
                                            ),
                                            0.6,
                                            CONFIG.color_white,
                                        ),
                                    GuiBuilder::new(MorphState::Rubber.to_string())
                                        .size(Value::Fixed(0.8), Value::Fixed(0.8))
                                        .texture(TEX_GUI_RUBBER, 0)
                                        .align(CENTER, CENTER)
                                        .margin(0.05, 0.05, 0.0, 0.0)
                                        .text(
                                            &format!(
                                                "{}",
                                                level_info.available_morphs[MorphState::Rubber]
                                            ),
                                            0.6,
                                            CONFIG.color_white,
                                        ),
                                    GuiBuilder::new(MorphState::Water.to_string())
                                        .size(Value::Fixed(0.8), Value::Fixed(0.8))
                                        .margin(0.05, 0.05, 0.0, 0.0)
                                        .texture(TEX_GUI_WATER, 0)
                                        .align(CENTER, CENTER)
                                        .text(
                                            &format!(
                                                "{}",
                                                level_info.available_morphs[MorphState::Water]
                                            ),
                                            0.6,
                                            CONFIG.color_white,
                                        ),
                                    GuiBuilder::new(MorphState::Bubble.to_string())
                                        .size(Value::Fixed(0.8), Value::Fixed(0.8))
                                        .margin(0.05, 0.05, 0.0, 0.0)
                                        .texture(TEX_GUI_BUBBLE, 0)
                                        .align(CENTER, CENTER)
                                        .text(
                                            &format!(
                                                "{}",
                                                level_info.available_morphs[MorphState::Bubble]
                                            ),
                                            0.6,
                                            CONFIG.color_white,
                                        ),
                                ]),
                        ]),
                    GuiBuilder::new("spacing").size(Value::Auto, Value::Auto),
                    GuiBuilder::new("bottom")
                        .size(Value::Auto, Value::Fixed(1.0))
                        .vertical()
                        .align(CENTER, BOTTOM)
                        .children(vec![GuiBuilder::new("tap")
                            .size(Value::Auto, Value::Auto)
                            .align(CENTER, CENTER)
                            .text("(Tap to start)", 0.5, CONFIG.color_white)]),
                ]),

            LevelPhase::Running => GuiBuilder::new("hud")
                .size(Value::Auto, Value::Auto)
                .padding(0.1, 0.1, 0.1, 0.1)
                .vertical()
                .children(vec![
                    GuiBuilder::new("top")
                        .size(Value::Auto, Value::Auto)
                        .align(LEFT, TOP)
                        .children(vec![GuiBuilder::new("pause")
                            .size(Value::Fixed(1.0), Value::Fixed(1.0))
                            .texture(TEX_GUI_MENU, 0)
                            .click(LevelEvent::Pause)]),
                    GuiBuilder::new("bottom")
                        .size(Value::Auto, Value::Auto)
                        .vertical()
                        .align(RIGHT, BOTTOM)
                        .children(vec![
                            GuiBuilder::new("morph_top")
                                .size(Value::Fixed(3.3), Value::Fixed(1.5))
                                .margin(0.0, 0.0, 0.0, 0.0)
                                .align(RIGHT, CENTER)
                                .children(vec![
                                    GuiBuilder::new(MorphState::Metal.to_string())
                                        .size(Value::Fixed(1.5), Value::Fixed(1.5))
                                        .margin(0.0, 0.22, 0.0, 0.0)
                                        .texture(TEX_GUI_METAL, 0)
                                        .fast_click(LevelEvent::InputMorph(MorphState::Metal))
                                        .align(CENTER, CENTER)
                                        .text(
                                            &format!(
                                                "{}",
                                                level_info.available_morphs[MorphState::Metal]
                                            ),
                                            1.0,
                                            if level_info.available_morphs[MorphState::Metal] > 0 {
                                                CONFIG.color_white
                                            } else {
                                                CONFIG.color_red
                                            },
                                        ),
                                    GuiBuilder::new(MorphState::Rubber.to_string())
                                        .size(Value::Fixed(1.5), Value::Fixed(1.5))
                                        .margin(0.0, 0.0, 0.0, 0.0)
                                        .texture(TEX_GUI_RUBBER, 0)
                                        .fast_click(LevelEvent::InputMorph(MorphState::Rubber))
                                        .align(CENTER, CENTER)
                                        .text(
                                            &format!(
                                                "{}",
                                                level_info.available_morphs[MorphState::Rubber]
                                            ),
                                            1.0,
                                            if level_info.available_morphs[MorphState::Rubber] > 0 {
                                                CONFIG.color_white
                                            } else {
                                                CONFIG.color_red
                                            },
                                        ),
                                ]),
                            GuiBuilder::new("morph_bot")
                                .size(Value::Fixed(3.175), Value::Fixed(1.5))
                                .margin(0.0, 0.86, 0.0, 0.0)
                                .align(RIGHT, CENTER)
                                .children(vec![
                                    GuiBuilder::new(MorphState::Water.to_string())
                                        .size(Value::Fixed(1.5), Value::Fixed(1.5))
                                        .margin(0.0, 0.22, 0.0, 0.0)
                                        .texture(TEX_GUI_WATER, 0)
                                        .fast_click(LevelEvent::InputMorph(MorphState::Water))
                                        .align(CENTER, CENTER)
                                        .text(
                                            &format!(
                                                "{}",
                                                level_info.available_morphs[MorphState::Water]
                                            ),
                                            1.0,
                                            if level_info.available_morphs[MorphState::Water] > 0 {
                                                CONFIG.color_white
                                            } else {
                                                CONFIG.color_red
                                            },
                                        ),
                                    GuiBuilder::new(MorphState::Bubble.to_string())
                                        .size(Value::Fixed(1.5), Value::Fixed(1.5))
                                        .margin(0.0, 0.0, 0.0, 0.0)
                                        .texture(TEX_GUI_BUBBLE, 0)
                                        .fast_click(LevelEvent::InputMorph(MorphState::Bubble))
                                        .align(CENTER, CENTER)
                                        .text(
                                            &format!(
                                                "{}",
                                                level_info.available_morphs[MorphState::Bubble]
                                            ),
                                            1.0,
                                            if level_info.available_morphs[MorphState::Bubble] > 0 {
                                                CONFIG.color_white
                                            } else {
                                                CONFIG.color_red
                                            },
                                        ),
                                ]),
                        ]),
                ]),

            LevelPhase::Finish => GuiBuilder::new(""),
        }
    } else {
        GuiBuilder::new("")
    }
}
