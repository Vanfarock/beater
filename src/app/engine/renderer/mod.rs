pub mod context;

use std::sync::Arc;

use anyhow::Result;
use context::Context;
use winit::window::Window;

pub struct Renderer {
    context: Context,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Result<Self> {
        let context = Context::new(window)?;

        Ok(Self { context })
    }
}
