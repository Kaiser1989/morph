//////////////////////////////////////////////////
// Using

use enum_map::{enum_map, Enum, EnumMap};
use log::info;
use nalgebra_glm::*;
use shrev::ReaderId;

use crate::game::ecs::resource::MorphState;
use crate::game::fx::GraphicsContext;
use crate::game::resource::{Events, Gui, InputContext, ResourceContext};
use crate::game::state::GameState;
use crate::game::StateEvent;

use super::gui;
use super::scene::Scene;

//////////////////////////////////////////////////
// Definition

pub struct LevelState {
    events: Events<LevelEvent>,
    reader: ReaderId<LevelEvent>,
    gui: EnumMap<LevelPhase, Gui<LevelEvent>>,
    scene: Scene,
    phase: LevelPhase,
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

#[derive(Debug, Clone, Copy, PartialEq, Enum)]
pub enum LevelPhase {
    Preview,
    Running,
    Finish,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemPhase {
    Events,
    Update,
}
impl Default for SystemPhase {
    fn default() -> Self {
        SystemPhase::Update
    }
}

//////////////////////////////////////////////////
// Implementation

impl LevelState {
    pub fn new() -> LevelState {
        let gui = enum_map! {
            LevelPhase::Preview => Gui::new(),
            LevelPhase::Running => Gui::new(),
            LevelPhase::Finish => Gui::new(),
        };
        let mut events = Events::new();
        let reader = events.register();
        let scene = Scene::new(events.register());
        let phase = LevelPhase::Preview;
        LevelState { gui, scene, events, reader, phase }
    }
}

impl GameState for LevelState {
    fn init(&mut self, resource: &ResourceContext) {
        // init level phase
        self.phase = LevelPhase::Preview;
        // init gui
        self.gui.iter_mut().for_each(|(phase, gui)| {
            gui.init(&gui::create(resource, phase));
        });
        // init scene
        self.scene.init(resource);
    }

    fn cleanup(&mut self, _resource: &ResourceContext) {
        // clear scene
        self.scene = Scene::new(self.events.register());
        // clear gui
        self.gui = enum_map! {
            LevelPhase::Preview => Gui::new(),
            LevelPhase::Running => Gui::new(),
            LevelPhase::Finish => Gui::new(),
        };
        // clear phase
        self.phase = LevelPhase::Preview;
    }

    fn handle_input(&mut self, input: &InputContext) {
        // handle back button
        if input.back() {
            self.events.write(LevelEvent::Pause);
        }

        // handle gui events
        self.gui[self.phase].handle_input(input, &mut self.events);

        // preview events   // TODO: Maybe change
        if let (LevelPhase::Preview, Some((_start, delta))) = (self.phase, input.drag()) {
            self.events.write(LevelEvent::MoveCamera(delta));
        }
    }

    fn update(&mut self, elapsed_time: f32, state_events: &mut Events<StateEvent>) {
        // update scene
        self.scene.update(elapsed_time, &mut self.events);

        // update delayed events
        self.events.update_delayed(elapsed_time);

        // handle events
        for event in self.events.read(&mut self.reader) {
            match event {
                // phase changing events
                LevelEvent::Start => {
                    info!("LevelEvent: Start");
                    self.phase = LevelPhase::Running;
                }
                LevelEvent::Pause => {
                    info!("LevelEvent: Pause");
                    state_events.write(StateEvent::LevelPause);
                }
                LevelEvent::Success if self.phase == LevelPhase::Running => {
                    info!("LevelEvent: Success");
                    self.phase = LevelPhase::Finish;
                    state_events.write(StateEvent::LevelSuccess);
                }
                LevelEvent::Failure if self.phase == LevelPhase::Running => {
                    info!("LevelEvent: Failure");
                    self.phase = LevelPhase::Finish;
                    state_events.write(StateEvent::LevelFailure);
                }
                _ => {}
            }
        }
    }

    fn draw(&mut self, graphics: &mut GraphicsContext) {
        // draw scene
        self.scene.draw(graphics);

        // draw gui
        self.gui[self.phase].draw(graphics);
    }

    fn create_device(&mut self, graphics: &mut GraphicsContext) {
        // adjust gui dimension
        self.gui.as_mut_slice().iter_mut().for_each(|gui| {
            gui.adjust_dimension(graphics.resolution());
        });
    }

    fn resize_device(&mut self, graphics: &mut GraphicsContext) {
        // adjust gui dimension
        self.gui.as_mut_slice().iter_mut().for_each(|gui| {
            gui.adjust_dimension(graphics.resolution());
        });
    }

    fn destroy_device(&mut self, _graphics: &mut GraphicsContext) {
        // adjust gui dimension
        self.gui.as_mut_slice().iter_mut().for_each(|gui| {
            gui.adjust_dimension(Vec2::zeros());
        });
    }
}

impl Default for LevelPhase {
    fn default() -> Self {
        LevelPhase::Preview
    }
}
