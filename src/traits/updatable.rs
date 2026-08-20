pub trait Updatable {
  fn update(&mut self, dt: f32, world_width: f32, world_height: f32);
}

pub trait UpdatableSlice {
  fn update_all(&mut self, dt: f32, world_width: f32, world_height: f32);
}


impl<T: Updatable> UpdatableSlice for [T] {
  fn update_all(&mut self, dt: f32, sw: f32, sh: f32) {
    for item in self.iter_mut() {
      item.update(dt, sw, sh);
    }
  }
}
