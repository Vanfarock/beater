mod app;

use anyhow::Result;
use app::App;
use winit::event_loop::EventLoop;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut app = App::default();
    let event_loop = EventLoop::new()?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
