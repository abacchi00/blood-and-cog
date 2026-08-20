use macroquad::prelude::*;

use crate::traits::{
  Renderable,
  Updatable,
  Expirable,
  Collidable, CollisionShape,
};
use crate::config::*;

pub struct Bullet {
  pub pos: Vec2,
  pub radius: f32, 
  pub collided: bool,
  speed: f32,
  dir: Vec2,
}

impl Bullet {
  pub fn new(start_pos: Vec2, target_pos: Vec2) -> Self {
    let dir = (target_pos - start_pos).normalize();

    Self {
      pos: start_pos,
      speed: BULLET_BASE_SPEED,
      dir,
      radius: BULLET_RADIUS,
      collided: false,
    }
  }

  pub fn is_within_bounds(&self) -> bool {
    // Large, reasonable world boundary box
    self.pos.x >= -5000.0
      && self.pos.x <= 5000.0
      && self.pos.y >= -5000.0
      && self.pos.y <= 5000.0
  }
}

impl Renderable for Bullet {
  fn render(&self) {
    draw_circle(self.pos.x, self.pos.y, self.radius, BULLET_COLOR);
  }
}

impl Updatable for Bullet {
  fn update(&mut self, dt: f32, _world_width: f32, _world_height: f32) {
    self.pos += self.dir * self.speed * dt;
  }
}

impl Expirable for Bullet {
  fn should_clean(&self) -> bool {
    self.collided || !self.is_within_bounds()
  }
}

impl Collidable for Bullet {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: self.radius }
  }
}
