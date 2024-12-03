mod renderer;
use anyhow::Result;
use renderer::Renderer;
use std::{collections::HashMap, sync::Arc};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowAttributes, WindowId};

pub struct Engine {
    renderers: HashMap<WindowId, Renderer>,
    windows: HashMap<WindowId, Arc<Window>>,
    primary_window_id: WindowId,
}

impl Engine {
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self> {
        let mut engine = Self {
            renderers: HashMap::default(),
            windows: HashMap::default(),
            primary_window_id: WindowId::dummy(),
        };

        let window_id = engine.create_window(event_loop, WindowAttributes::default())?;
        engine.primary_window_id = window_id;

        Ok(engine)
    }

    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        attributes: WindowAttributes,
    ) -> Result<WindowId> {
        let window = Arc::new(event_loop.create_window(attributes)?);
        let window_id = window.id();
        self.windows.insert(window_id, window.clone());

        let renderer = Renderer::new(window)?;
        self.renderers.insert(window_id, renderer);

        Ok(window_id)
    }
    pub fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                if window_id == self.primary_window_id {
                    event_loop.exit();
                } else {
                    self.windows.remove(&window_id);
                    self.renderers.remove(&window_id);
                }
            }
            _ => {}
        }
    }
}
