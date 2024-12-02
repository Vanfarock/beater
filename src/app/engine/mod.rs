mod renderer;
use anyhow::Result;
use std::{collections::HashMap, sync::Arc};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct Engine {
    renderers: HashMap<WindowId, Arc<Window>>,
    windows: HashMap<WindowId, Arc<Window>>,
    primary_window: Option<WindowId>,
}

impl Engine {
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self> {
        Ok(Self {
            renderers: Default::default(),
            windows: Default::default(),
            primary_window: None,
        })
    }
}
