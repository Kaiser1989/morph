//////////////////////////////////////////////////
// Using

use log::info;
use nalgebra_glm::*;

use crate::game::config::Config;
use crate::game::ecs::resource::MorphState;
use crate::game::ecs::system::{
    AnimationSystem, InputCameraSystem, InputMorphSystem, LifetimeSystem, OutputSystem, PhysicFollowSystem, PhysicForceSystem, PhysicInteractionSystem, PhysicReadSystem, PhysicSyncSystem,
    PhysicUpdateSystem, PhysicWriteSystem, RenderSystem, StoryInteractionSystem, StoryMorphAnimationSystem, StoryMorphSystem,
};
use crate::game::game_state::{GameState, GameStateData, GameStateEvent};
use crate::game::resource::gui::*;
use crate::game::resource::Events;
use crate::game::{fx::*, ResourceContext, SceneBuilder};
use crate::game::{GuiBuilder, StateEvent};

//////////////////////////////////////////////////
// Definition

pub struct LevelState {
    data: GameStateData<LevelEvent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LevelEvent {
    Start,
    Pause,
    Success,
    Failure,
    InputMorph(MorphState),
    MoveCamera(Vec2),
}

//////////////////////////////////////////////////
// Implementation

impl GameStateEvent for LevelEvent {}

impl LevelState {
    pub fn new(config: &Config) -> Self {
        Self { data: GameStateData::new(config) }
    }
}

impl GameState for LevelState {
    type Event = LevelEvent;

    fn data(&mut self) -> &mut GameStateData<Self::Event> {
        &mut self.data
    }

    fn handle_event(&mut self, event: Self::Event, state_events: &mut Events<StateEvent>) {
        match event {
            // phase changing events
            LevelEvent::Start => {
                info!("LevelEvent: Start");
            }
            LevelEvent::Pause => {
                info!("LevelEvent: Pause");
                state_events.write(StateEvent::LevelPause);
            }
            LevelEvent::Success => {
                info!("LevelEvent: Success");
                state_events.write(StateEvent::LevelSuccess);
            }
            LevelEvent::Failure => {
                info!("LevelEvent: Failure");
                state_events.write(StateEvent::LevelFailure);
            }
            _ => {}
        }
    }

    fn scene(&self, _resource: &ResourceContext) -> Option<SceneBuilder> {
        Some(
            SceneBuilder::new()
                .update_system::<InputMorphSystem>()
                .update_system::<InputCameraSystem>()
                .update_system::<PhysicSyncSystem>()
                .update_system::<PhysicForceSystem>()
                .update_system::<PhysicWriteSystem>()
                .update_system::<PhysicUpdateSystem>()
                .update_system::<PhysicFollowSystem>()
                .update_system::<PhysicInteractionSystem>()
                .update_system::<PhysicReadSystem>()
                .update_system::<StoryInteractionSystem>()
                .update_system::<StoryMorphSystem>()
                .update_system::<StoryMorphAnimationSystem>()
                .update_system::<AnimationSystem>()
                .update_system::<LifetimeSystem>()
                .update_system::<OutputSystem>()
                .render_system::<RenderSystem>(),
        )
    }

    fn gui(&self, resource: &ResourceContext) -> Option<GuiBuilder<Self::Event>> {
        let config = self.data.config();
        let level_info = resource.level_info().unwrap();
        Some(GuiBuilder::new("hud").size(Value::Auto, Value::Auto).padding(0.1, 0.1, 0.1, 0.1).vertical().children(vec![
            GuiBuilder::new("top").size(Value::Auto, Value::Auto).align(LEFT, TOP).children(vec![
                GuiBuilder::new("pause").size(Value::Fixed(1.0), Value::Fixed(1.0)).texture(TEX_GUI_MENU, 0).click(LevelEvent::Pause),
            ]),
            GuiBuilder::new("bottom").size(Value::Auto, Value::Auto).vertical().align(RIGHT, BOTTOM).children(vec![
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
                                    &format!("{}", level_info.available_morphs[MorphState::Metal]),
                                    1.0,
                                    if level_info.available_morphs[MorphState::Metal] > 0 {
                                        config.color_white
                                    } else {
                                        config.color_red
                                    },
                                ),
                            GuiBuilder::new(MorphState::Rubber.to_string())
                                .size(Value::Fixed(1.5), Value::Fixed(1.5))
                                .margin(0.0, 0.0, 0.0, 0.0)
                                .texture(TEX_GUI_RUBBER, 0)
                                .fast_click(LevelEvent::InputMorph(MorphState::Rubber))
                                .align(CENTER, CENTER)
                                .text(
                                    &format!("{}", level_info.available_morphs[MorphState::Rubber]),
                                    1.0,
                                    if level_info.available_morphs[MorphState::Rubber] > 0 {
                                        config.color_white
                                    } else {
                                        config.color_red
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
                                    &format!("{}", level_info.available_morphs[MorphState::Water]),
                                    1.0,
                                    if level_info.available_morphs[MorphState::Water] > 0 {
                                        config.color_white
                                    } else {
                                        config.color_red
                                    },
                                ),
                            GuiBuilder::new(MorphState::Bubble.to_string())
                                .size(Value::Fixed(1.5), Value::Fixed(1.5))
                                .margin(0.0, 0.0, 0.0, 0.0)
                                .texture(TEX_GUI_BUBBLE, 0)
                                .fast_click(LevelEvent::InputMorph(MorphState::Bubble))
                                .align(CENTER, CENTER)
                                .text(
                                    &format!("{}", level_info.available_morphs[MorphState::Bubble]),
                                    1.0,
                                    if level_info.available_morphs[MorphState::Bubble] > 0 {
                                        config.color_white
                                    } else {
                                        config.color_red
                                    },
                                ),
                        ]),
                ]),
        ]))
    }
}
