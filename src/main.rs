mod engine;
use engine::{*};

use winit::event_loop::EventLoop;
use anyhow::Result;

fn main() -> Result<()> {
    let mut game = WindowGame::new();
    
    let event_loop = EventLoop::new()?;
    
    let _ = event_loop.run_app(&mut game);
    
    Ok(())
}
