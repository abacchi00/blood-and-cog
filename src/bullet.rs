use macroquad::prelude::*;

use crate::traits::{Renderable, Updatable, Expirable};

const BULLET_RADIUS: f32 = 4.0;
const BULLET_COLOR: Color = WHITE;
const BULLET_VEL: f32 = 1000.0; // pixels-per-second with dt

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

  pub fn collides_with(&self, obj_pos: Vec2, obj_radius: f32) -> bool {
    let dx = self.pos.x - obj_pos.x;
    let dy = self.pos.y - obj_pos.y;
    
    let c_sqrd = (dx * dx) + (dy * dy);
    
    let radius_sum = self.radius + obj_radius;
    let radius_sum_sqrd = radius_sum * radius_sum;
    
    c_sqrd < radius_sum_sqrd
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
