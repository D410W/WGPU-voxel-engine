#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VoxelFace {
  pub x: i16, // X, Y, Z coordinates
  pub y: i16, // X, Y, Z coordinates
  pub z: i16, // X, Y, Z coordinates
  pub face: u8,   // Up, Down, Left, Back, etc.
  pub block_id: u8,  // block id
}

impl VoxelFace {
  // Describes how this data looks in memory so the GPU can read it
  pub fn desc() -> wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<VoxelFace>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Instance,
      attributes: &[
        wgpu::VertexAttribute { format: wgpu::VertexFormat::Sint16, offset: 0, shader_location: 0 }, // pos x
        wgpu::VertexAttribute { format: wgpu::VertexFormat::Sint16, offset: 2, shader_location: 1 }, // pos y
        wgpu::VertexAttribute { format: wgpu::VertexFormat::Sint16, offset: 4, shader_location: 2 }, // pos z
        wgpu::VertexAttribute { format: wgpu::VertexFormat::Uint8, offset: 6, shader_location: 3 }, // face dir
        wgpu::VertexAttribute { format: wgpu::VertexFormat::Uint8, offset: 7, shader_location: 4 }, // block id
      ],
    }
  }
}
