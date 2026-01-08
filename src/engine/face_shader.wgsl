struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0) 
var<uniform> camera: CameraUniform;

struct InstanceInput {
  @location(0) pos_x: i32,
  @location(1) pos_y: i32,
  @location(2) pos_z: i32,
  @location(3) face_dir: u32,
  @location(4) block_id: u32,
};

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
  @builtin(vertex_index) in_vertex_index: u32,
  instance: InstanceInput,
) -> VertexOutput {
  var out: VertexOutput;
  
  let base_pos = vec3<f32>(f32(instance.pos_x), f32(instance.pos_y), f32(instance.pos_z));
  
  let offsets_2d = array<array<f32, 2>, 4>(array<f32,2>(0.0, 0.0),
                                           array<f32,2>(0.0, 1.0),
                                           array<f32,2>(1.0, 0.0),
                                           array<f32,2>(1.0, 1.0));
  
  var offset = vec3<f32>(0.0, 0.0, 0.0);
  switch instance.face_dir {
    case 0: { // up
      offset = vec3<f32>(offsets_2d[in_vertex_index][0] - 0.5,
                         0.5,
                         offsets_2d[in_vertex_index][1] - 0.5);
    } case 1: { // left
      offset = vec3<f32>(-0.5,
                         offsets_2d[in_vertex_index][0] - 0.5,
                         offsets_2d[in_vertex_index][1] - 0.5);
    } case 2: { // front
      offset = vec3<f32>(offsets_2d[in_vertex_index][0] - 0.5,
                         offsets_2d[in_vertex_index][1] - 0.5,
                         0.5);
    } case 3: { // right
      offset = vec3<f32>(0.5,
                         offsets_2d[in_vertex_index][0] - 0.5,
                         offsets_2d[in_vertex_index][1] - 0.5);
    } case 4: { // back
    } case 5: { // down
      offset = vec3<f32>(offsets_2d[in_vertex_index][0] - 0.5,
                         offsets_2d[in_vertex_index][1] - 0.5,
                         -0.5);
    } default: { // error
    }
  };
  
  
  let world_pos = vec4<f32>(base_pos + offset, 1.0);
  
  out.clip_position = camera.view_proj * world_pos;
  
  switch instance.block_id {
    case 1: {
      out.color = vec3<f32>(1.0, 0.0, 0.0);
    }
    case 2: {
      out.color = vec3<f32>(0.0, 1.0, 0.0);
    }
    case 3: {
      out.color = vec3<f32>(0.0, 0.0, 1.0);
    }
    default: {
      out.color = vec3<f32>(1.0, 1.0, 1.0);
    }
  }
  
  return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  return vec4<f32>(in.color, 1.0);
}
