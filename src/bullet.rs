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
  vel: f32,
  dir: Vec2,
}

impl Bullet {
  pub fn new(start_pos: Vec2, target_pos: Vec2) -> Self {
    let dir = (target_pos - start_pos).normalize();

    Self {
      pos: start_pos,
      vel: BULLET_VEL,
      dir,
      radius: BULLET_RADIUS,
      collided: false,
    }
  }

  pub fn is_within_bounds(&self) -> bool {
    let sw = screen_width();
    let sh = screen_height();
    
    self.pos.x >= 0.0
      && self.pos.x <= sw
      && self.pos.y >= 0.0
      && self.pos.y <= sh
  }
}

impl Renderable for Bullet {
  fn render(&self) {
    draw_circle(self.pos.x, self.pos.y, self.radius, BULLET_COLOR);
  }
}

impl Updatable for Bullet {
  fn update(&mut self, dt: f32, _world_width: f32, _world_height: f32) {
    self.pos += self.dir * self.vel * dt;
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
