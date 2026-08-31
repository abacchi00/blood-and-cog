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
    let trail_length = self.speed * 0.04; 
    let tail = self.pos - self.dir * trail_length;

    // External orange trail
    draw_line(
      self.pos.x, self.pos.y, 
      tail.x, tail.y, 
      3.0, 
      Color::new(1.0, 0.6, 0.0, 0.6)
    );

    // Thinner white internal nucleus
    draw_line(
      self.pos.x, self.pos.y, 
      tail.x, tail.y, 
      1.5, 
      WHITE
    );
    
    // Bullet end
    draw_circle(self.pos.x, self.pos.y, 2.0, Color::new(1.0, 1.0, 0.5, 0.8));
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
