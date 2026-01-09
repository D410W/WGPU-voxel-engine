pub struct Camera3D {
  pub position: glam::Vec3, // 3 x f32
  pub rotation_x: f32, // up-down
  pub rotation_y: f32, // left-right
}

impl Camera3D {
  pub fn new(position: glam::Vec3) -> Self {
    Camera3D{
      position,
      rotation_y: 0.0,
      rotation_x: 0.0,
    }
  }
  
  fn get_right_vector(&self) -> glam::Vec3 {
    let yaw = self.rotation_y.to_radians();
    // let pitch = self.rotation_x.to_radians();
    
    glam::Vec3::new(
      yaw.cos(),
      0.0,
      -yaw.sin(),
    ).normalize()
  }
  
  fn get_forward_vector(&self) -> glam::Vec3 {
    let yaw = self.rotation_y.to_radians();
    let pitch = self.rotation_x.to_radians();
    
    glam::Vec3::new(
      yaw.sin() * pitch.cos(),
      -pitch.sin(),
      yaw.cos() * pitch.cos(),
    ).normalize()
  }
  
  fn get_up_vector(&self) -> glam::Vec3 {
    let yaw = self.rotation_y.to_radians();
    let pitch = self.rotation_x.to_radians();
    
    glam::Vec3::new(
      yaw.sin() * pitch.sin(),
      pitch.cos(),
      yaw.cos() * pitch.sin(),
    ).normalize()
  }
  
  pub fn rotate_xy(&mut self, delta: (f64, f64)) {
    let delta = (delta.0 as f32, delta.1 as f32);
    const SENS: f32 = 10.0/100.0;
    const RIGHT_ANGLE: f32 = 89.0;
    
    self.rotation_y -= delta.0 as f32 * SENS;
    self.rotation_y %= 360.0;
    
    if self.rotation_x - delta.1 < -RIGHT_ANGLE {
      self.rotation_x = -RIGHT_ANGLE;
    } else if self.rotation_x - delta.1 > RIGHT_ANGLE {
      self.rotation_x = RIGHT_ANGLE;
    } else {
      self.rotation_x -= delta.1 as f32 * SENS;
    }
    
  }
  
  pub fn move_relative(&mut self, delta: [f32;3], unit: f32) {
    let delta = glam::Vec3::from_array(delta);
    self.position += ( delta.x * self.get_right_vector() +
                       delta.y * self.get_up_vector() +
                       delta.z * self.get_forward_vector() ) * unit;
  }
  
  pub fn move_absolute(&mut self, delta: [f32;3], unit: f32) {
    self.position += glam::Vec3::from_array(delta) * unit;
  }
  
  pub fn get_view(&mut self) -> glam::Mat4 {
    // (eye_pos, target_pos, up_vector)
    glam::Mat4::look_at_rh(
      self.position,
      self.position + glam::Mat3::from_rotation_y(self.rotation_y.to_radians()) *
                      glam::Mat3::from_rotation_x(self.rotation_x.to_radians()) *
                      glam::vec3(0.0, 0.0, -1.0),
      glam::vec3(0.0, 1.0, 0.0)
    )
  }
}
