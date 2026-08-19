pub trait Updatable {
  fn update(&mut self, dt: f32, world_width: f32, world_height: f32);
}
