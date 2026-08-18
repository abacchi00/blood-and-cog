use macroquad::prelude::*;

const BULLET_RADIUS: f32 = 4.0;
const BULLET_COLOR: Color = WHITE;
const BULLET_VEL: f32 = 1000.0; // pixels-per-second with dt

pub struct Bullet {
  pub pos: Vec2,
  vel: f32,
  dir: Vec2,
  pub radius: f32, 
}

impl Bullet {
  pub fn new(start_pos: Vec2, target_pos: Vec2) -> Self {
    let dir = (target_pos - start_pos).normalize();

    Self {
      pos: start_pos,
      vel: BULLET_VEL,
      dir,
      radius: BULLET_RADIUS,
    }
  }

  pub fn update_pos(&mut self, dt: f32) {
    self.pos += self.dir * self.vel * dt;
  }

  pub fn render(&self) {
    draw_circle(self.pos.x, self.pos.y, self.radius, BULLET_COLOR);
  }
}