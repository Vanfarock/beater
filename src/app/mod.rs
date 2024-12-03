mod engine;

use engine::Engine;
use winit::{
    application::ApplicationHandler,
    event,
    event_loop::ActiveEventLoop,
    window::{Fullscreen, WindowAttributes, WindowId},
};

#[derive(Default)]
pub struct App {
    engine: Option<Engine>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.engine = Some(Engine::new(event_loop).unwrap());
        if let Some(engine) = self.engine.as_mut() {
            let _ = engine
                .create_window(
                    &event_loop,
                    WindowAttributes::default().with_title("Secondary Window"),
                )
                .unwrap();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: event::WindowEvent,
    ) {
        if let Some(engine) = self.engine.as_mut() {
            engine.window_event(event_loop, window_id, event);
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.engine = None;
    }
}
