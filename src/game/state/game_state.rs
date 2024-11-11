//////////////////////////////////////////////////
// Using

use crate::game::{config::Config, Events, GraphicsContext, Gui, GuiBuilder, InputContext, ResourceContext, Scene, SceneBuilder, StateEvent};
use nalgebra_glm::Vec2;
use shrev::ReaderId;
use specs::{RunNow, System, World};

//////////////////////////////////////////////////
// GameState

pub trait GameStateEvent: Sized + Clone + Send + Sync + 'static {}

pub trait GameState {
    type Event: GameStateEvent;

    fn data(&mut self) -> &mut GameStateData<Self::Event>;

    fn handle_event(&mut self, event: Self::Event, state_events: &mut Events<StateEvent>);

    fn gui(&self, _resource: &ResourceContext) -> Option<GuiBuilder<Self::Event>> {
        None
    }

    fn scene(&self, _resource: &ResourceContext) -> Option<SceneBuilder> {
        None
    }
}

pub trait InternalGameState {
    fn init(&mut self, resource: &ResourceContext);

    fn cleanup(&mut self, _resource: &ResourceContext);

    fn handle_input(&mut self, input: &InputContext);

    fn update(&mut self, elapsed_time: f32, state_events: &mut Events<StateEvent>);

    fn draw(&mut self, graphics: &GraphicsContext);

    fn create_device(&mut self, graphics: &GraphicsContext);

    fn resize_device(&mut self, graphics: &GraphicsContext);

    fn destroy_device(&mut self, _graphics: &GraphicsContext);

    fn parent_update(&self) -> bool {
        false
    }

    fn parent_draw(&self) -> bool {
        false
    }
}

impl<G> InternalGameState for G
where
    G: GameState,
{
    fn init(&mut self, resource: &ResourceContext) {
        if let Some(builder) = self.gui(resource) {
            let data = self.data();
            data.gui = Some(Gui::new(data.config()).init(builder));
        }
        if let Some(builder) = self.scene(resource) {
            let data = self.data();
            data.scene = Some(Scene::new(data.config()).init(builder));
        }
    }

    fn cleanup(&mut self, _resource: &ResourceContext) {
        let data = self.data();
        if let Some(gui) = data.gui.as_mut() {
            gui.cleanup();
        }
        if let Some(scene) = data.scene.as_mut() {
            scene.cleanup();
        }
    }

    fn handle_input(&mut self, input: &InputContext) {
        let data = self.data();
        if let Some(gui) = data.gui.as_mut() {
            gui.handle_input(input, &mut data.events);
        }
        if let Some(scene) = data.scene.as_mut() {
            scene.handle_input(input, &mut data.events);
        }
    }

    fn update(&mut self, elapsed_time: f32, state_events: &mut Events<StateEvent>) {
        let data = self.data();
        data.events.update_delayed(elapsed_time);

        for event in data.events.read(&mut data.reader) {
            self.handle_event(event, state_events);
        }

        let data = self.data();
        if let Some(scene) = data.scene.as_mut() {
            scene.update(elapsed_time, &mut data.events);
        }
    }

    fn draw(&mut self, graphics: &GraphicsContext) {
        let data = self.data();
        if let Some(gui) = data.gui.as_mut() {
            gui.draw(graphics);
        }
        if let Some(scene) = data.scene.as_mut() {
            scene.draw(graphics);
        }
    }

    fn create_device(&mut self, graphics: &GraphicsContext) {
        let data = self.data();
        if let Some(gui) = data.gui.as_mut() {
            gui.adjust_dimension(graphics.lock().unwrap().resolution());
        }
    }

    fn resize_device(&mut self, graphics: &GraphicsContext) {
        let data = self.data();
        if let Some(gui) = data.gui.as_mut() {
            gui.adjust_dimension(graphics.lock().unwrap().resolution());
        }
    }

    fn destroy_device(&mut self, _graphics: &GraphicsContext) {
        let data = self.data();
        if let Some(gui) = data.gui.as_mut() {
            gui.adjust_dimension(Vec2::zeros());
        }
    }

    fn parent_update(&self) -> bool {
        false
    }

    fn parent_draw(&self) -> bool {
        false
    }
}

//////////////////////////////////////////////////
// GameSystemData

pub struct GameStateData<E: GameStateEvent> {
    config: Config,
    events: Events<E>,
    reader: ReaderId<E>,
    gui: Option<Gui<E>>,
    scene: Option<Scene<E>>,
}

impl<E: GameStateEvent> GameStateData<E> {
    pub fn new(config: &Config) -> Self {
        let config = config.clone();
        let mut events = Events::new();
        let reader = events.register();
        let gui = None;
        let scene = None;
        Self { config, events, reader, gui, scene }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}

//////////////////////////////////////////////////
// GameSystem

pub trait GameSystem<'a> {
    fn init(&mut self, world: &mut World);

    fn update(&mut self, world: &'a World);
}

impl<'a, S> GameSystem<'a> for S
where
    S: System<'a> + 'static,
{
    fn init(&mut self, world: &mut World) {
        RunNow::setup(self, world);
    }

    fn update(&mut self, world: &'a World) {
        self.run_now(world);
    }
}
