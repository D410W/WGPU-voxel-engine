use wgpu::{*};
use winit::{
  application::ApplicationHandler,
  event::*,
  event_loop::{ActiveEventLoop, EventLoop},
  window::{Window, WindowId},
};

use std::time::{Duration, Instant};
use anyhow::Result;

use crate::WindowState;

/// WindowGame. An abstraction layer between the User and the Engine. Is responsible for the game loop, rendering and input catching.
pub struct WindowGame {
  window_state: Option<WindowState>,  
  
  // game stuff
  current_time: Instant,
  fixed_time_step: Duration,
  accumulator: Duration,
  
  frame: u32,
  
  // engine: Engine<GS>,
  // game_state: GS,
  
}

impl WindowGame {
  pub fn new() -> Result<Self> {    
    // let (cols, rows) = (32, 18);
    
    // let mut eng = Engine::<GS>::new((cols, rows));
    // let gs = GameState::new(&mut eng);
    
    let target_fps = 60;
    
    Ok(Self{
      window_state: None,
      
      current_time: Instant::now(),
      fixed_time_step: Duration::from_secs_f32(1.0 / target_fps as f32),
      accumulator: Duration::new(0,0),
      
      frame: 0,
      
      // engine: eng,
      // game_state: gs,
    })
  }
  
  fn draw(&mut self) -> Result<(), wgpu::SurfaceError> {
  
    let ws = self.window_state.as_mut().unwrap();
    
    let drawable = ws.surface.get_current_texture()?; // SurfaceTexture
    let image_view_descriptor = TextureViewDescriptor::default();
    let image_view = drawable.texture.create_view(&image_view_descriptor); // TextureView
    
    let command_enconder_descriptor = CommandEncoderDescriptor{ // CommandEncoderDescriptor
      label: Some("Render Encoder"),
    };
    let mut command_encoder = ws.device.create_command_encoder(&command_enconder_descriptor); // CommandEncoder
        
    if true {
      
      // println!("redraw");
      // let projection = glam::Mat4::perspective_rh(90.0/3.1415926535/2.0, 16.0/9.0, 0.0, 10000.0); // (fov, aspect_ratio, near_z, far_z)
      // let view = glam::Mat4::look_at_rh(glam::vec3(0.0, 0.0, 0.0), glam::vec3(0.0, 0.0, 0.0), glam::vec3(0.0, 1.0, 0.0)); // (eye_pos, target_pos, up_vector)
      
      // let view_proj = projection * view;
      // background update
      ws.voxelface_data.clear();
      
      ws.voxelface_data.push(crate::VoxelFace {
        x: 0,
        y: 0,
        z: 0,
        face: 2, // front
        block_id: 1,
      });
      
      ws.voxelface_data.push(crate::VoxelFace {
        x: 1,
        y: 0,
        z: 0,
        face: 2, // front
        block_id: 2,
      });
      
      // check if buffer is big enough
      let needed_size = (ws.voxelface_data.len() * std::mem::size_of::<crate::VoxelFace>()) as u64;
      if needed_size > ws.voxelface_instance_buffer.size() {
        // println!("Warning: resizing background buffer!");
        ws.voxelface_instance_buffer = ws.device.create_buffer(&wgpu::BufferDescriptor {
          label: Some("Instance Buffer"),
          size: needed_size * 2, // Grow x2
          usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
          mapped_at_creation: false,
        });
      }
      
      // Upload to GPU
      ws.queue.write_buffer(&ws.voxelface_instance_buffer, 0, bytemuck::cast_slice(&ws.voxelface_data));
      ws.num_voxelface_instances = ws.voxelface_data.len() as u32;
    }
    
    // preparing the render pass
    
    let color_attachment = RenderPassColorAttachment{
      view: &image_view,
      resolve_target: None,
      depth_slice: None,
      ops: Operations{
        load: LoadOp::Clear(Color{
          r: 0.0,
          g: 0.0,
          b: 0.0,
          a: 0.0,
        }),
        store: StoreOp::Store,
      },
    };
  
    let render_pass_descriptor = RenderPassDescriptor{
      label: Some("Render Pass"),
      color_attachments: &[Some(color_attachment)],
      depth_stencil_attachment: None,
      occlusion_query_set: None,
      timestamp_writes: None,
      multiview_mask: None,
    };
  
    // render pass
    {
    
      let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
      
      // background
      render_pass.set_pipeline(&ws.voxelface_pipeline);
      render_pass.set_vertex_buffer(0, ws.voxelface_instance_buffer.slice(..));
      // draw 4 vertices
      render_pass.draw(0..4, 0..ws.num_voxelface_instances);

    }
    
    // ws.text_atlas.trim();
    
    ws.queue.submit(std::iter::once(command_encoder.finish()));
    
    drawable.present();
  
    Ok(())
  }
  
  // fn game_step(&mut self, event_loop: &ActiveEventLoop) {
  //   // game loop logic is placed here
    
  //   self.engine.frame_counter += 1;
    
  //   if !(self.game_state.should_run()) {
  //     event_loop.exit();
      
  //     return;
  //   }
    
  //   self.engine.inp_dis.dispatch(&mut self.engine.inp_man, &mut self.game_state);
    
  //   self.game_state.update(&mut self.engine);
    
  //   self.engine.inp_man.cycle_events();
    
  // }
  
}

impl ApplicationHandler for WindowGame {
  
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {    
    let attributes = Window::default_attributes()
      .with_title("ASCII Engine")
      .with_transparent(false)
      .with_maximized(true)
      .with_active(true);
  
    let window = event_loop.create_window(attributes).unwrap();
    
    let state_result = pollster::block_on(WindowState::new(window.into()));
    match state_result {
      Ok(win_state) => self.window_state = Some(win_state),
      Err(e) => {
        eprintln!("Error initializing GPU: {}", e);
      },
    };
  }
  
  fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        println!("The close button was pressed.");
        event_loop.exit();
      },
      WindowEvent::Resized(new_size) => {
        if let Some(ws) = &mut self.window_state {
          ws.resize(new_size);
        }
      },
      WindowEvent::KeyboardInput{ device_id: _id, event: _event, is_synthetic: synth } => {
        if synth { return; }
        
        // self.engine.inp_man.process_winit_key(event);
      },
      WindowEvent::RedrawRequested => {
        
        let new_time = Instant::now();
        let frame_time = new_time - self.current_time;
        self.current_time = new_time;
        self.accumulator += frame_time;
        
        let runs_update = self.accumulator >= self.fixed_time_step;
        
        while self.accumulator >= self.fixed_time_step {
          // self.game_step(event_loop);
          
          self.accumulator -= self.fixed_time_step;
        }
        
        if runs_update {
          self.frame += 1;
        }
        
        match self.draw() {
          Ok(_) => {}
          Err(SurfaceError::Lost) => { // If the swapchain is lost (e.g. driver update, monitor unplugged), recreate it
            if let Some(ws) = &mut self.window_state {
              ws.resize(ws.size);
              // self.engine.db.text_changed = true;
            }
          },
          Err(SurfaceError::OutOfMemory) => event_loop.exit(), // The system is out of memory, we should quit
          Err(e) => eprintln!("{:?}", e), // All other errors (Outdated, Timeout) should be resolved by the next frame
        }        
        // end of game logic
        
        // Queue a RedrawRequested event.
        if let Some(ws) = &mut self.window_state {
          ws.render(); // calls window.request_redraw()
        }
      }
      _ => (),
    }
  }
  
}
