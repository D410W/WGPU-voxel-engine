use std::sync::Arc;
use winit::{
  application::ApplicationHandler,
  event::*,
  event_loop::{ActiveEventLoop, EventLoop},
  window::{Window, WindowId},
};

use anyhow::Result;

// WindowState. Holds the objects linked to the wgpu API.
pub struct WindowState {
  // window fields
  pub surface: wgpu::Surface<'static>,
  pub device: wgpu::Device,
  pub queue: wgpu::Queue,
  pub config: wgpu::SurfaceConfiguration,
  pub size: winit::dpi::PhysicalSize<u32>,
  pub window: Arc<Window>,
  
  // voxel fields
  pub voxelface_pipeline: wgpu::RenderPipeline,
  pub voxelface_instance_buffer: wgpu::Buffer,
  pub num_voxelface_instances: u32,
  pub voxelface_data: Vec<crate::VoxelFace>,
  
  // camera fields
  pub camera_buffer: wgpu::Buffer,
  pub camera_bind_group: wgpu::BindGroup,
}

impl WindowState {

  pub async fn new(window: Arc<Window>) -> Result<Self> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
      backends: wgpu::Backends::PRIMARY, // only vulkan, dx12 and metal
                                         // backends: wgpu::Backends::VULKAN | wgpu::Backends::DX12 | wgpu::Backends::METAL, 
      ..Default::default()
    });
    
    let surface = instance.create_surface(window.clone())?;
    
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::default(),
      compatible_surface: Some(&surface),
      force_fallback_adapter: false,
    }).await?;
    
    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        label: Some("WGPU Device"),
        memory_hints: wgpu::MemoryHints::default(),
        required_features: wgpu::Features::default(),
        required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        trace: wgpu::Trace::Off,
    }).await?;
    
    // configuring surface
    let caps = surface.get_capabilities(&adapter);
    let surface_format = caps.formats[0];

    let size = window.inner_size();

    let surface_config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: wgpu::PresentMode::Fifo,
      alpha_mode: caps.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2,
    };
    
    surface.configure(&device, &surface_config);
    
    // camera fields
    let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Camera Buffer"),
        size: 64, // 64 bytes = a 4x4 f32 matrix
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    
    let camera_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("Camera Bind Group Layout"),
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::VERTEX, // Only the vertex shader needs the camera
          ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
          },
          count: None,
        },
      ],
    });
    
    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      layout: &camera_bind_group_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: camera_buffer.as_entire_binding(),
        },
      ],
      label: Some("camera_bind_group"),
    });
    
    // voxel drawing setup
    let shader = device.create_shader_module(wgpu::include_wgsl!("face_shader.wgsl"));
    
    let render_pipeline_layout = device.create_pipeline_layout(
      &wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&camera_bind_group_layout],
        immediate_size: 0,
      }
    );
    
    let voxelface_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Voxel Face Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        buffers: &[crate::VoxelFace::desc()], // Describe our struct
      },
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleStrip, // 4 verts per rect
        ..Default::default()
      },
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        targets: &[Some(wgpu::ColorTargetState {
          format: surface_format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::ALL,
        })],
      }),
      multiview_mask: None,
      cache: None,
    });
    
    let voxelface_instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Voxel Face Instance Buffer"),
        size: (std::mem::size_of::<crate::VoxelFace>() * 5000) as u64, // 5000 is the maximum size
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    
    Ok(WindowState{
      surface,
      device,
      queue,
      config: surface_config,
      size,
      window,
      
      voxelface_pipeline,
      voxelface_instance_buffer,
      num_voxelface_instances: 0,
      voxelface_data: Vec::with_capacity(5000),
      
      camera_buffer,
      camera_bind_group,
    })
  }
  
  pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
    
      self.size = new_size;
      self.config.width = new_size.width;
      self.config.height = new_size.height;
      
      // self.view_port.update(
      //   &self.queue, 
      //   glyphon::Resolution { width: new_size.width, height: new_size.height }
      // );
      
      // final config
      self.surface.configure(&self.device, &self.config);
      
      // self.text_atlas.trim();
    }
  }
  
  pub fn render(&mut self) {
    self.window.request_redraw();
  }
    
}
