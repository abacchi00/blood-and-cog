use macroquad::prelude::*;

use crate::config::*;
use crate::traits::{Collidable, CollisionShape, Renderable};
use crate::alt_shapes::BorderedCircle;

pub struct Player {
  pub pos: Vec2,
  pub life: f32,
  // max_life: f32,
  take_hit_cooldown: f32, // seconds
  pub cogs_count: i32,
}

impl Player {
  pub fn new(pos: Vec2) -> Self {
    Self {
      pos,
      life: PLAYER_BASE_LIFE,
      // max_life: PLAYER_BASE_LIFE,
      take_hit_cooldown: 0.0,
      cogs_count: 0,
    }
  }

  pub fn update(&mut self, dt: f32) {
    if self.take_hit_cooldown > 0.0 {
      self.take_hit_cooldown -= dt;
      self.take_hit_cooldown = self.take_hit_cooldown.max(0.0);
    }
  }

  pub fn take_hit(&mut self) {
    if self.take_hit_cooldown <= 0.0 {
      self.life -= 20.0;
      self.take_hit_cooldown = PLAYER_INVULNERABILITY_DURATION;
    }
  }

  pub fn is_alive(&self) -> bool {
    self.life > 0.0
  }

  pub fn calculate_movement_delta(&self, input_dir: Vec2, dt: f32) -> Vec2 {
    let mut dx = input_dir.x;
    let mut dy = input_dir.y;

    if dx != 0.0 && dy != 0.0 {
      let normalization = 1.0f32 / (2.0f32).sqrt();
      dx *= normalization;
      dy *= normalization;
    }

    vec2(dx * PLAYER_BASE_SPEED * dt, dy * PLAYER_BASE_SPEED * dt)
  }

  pub fn pick_cog(&mut self) {
    self.cogs_count += 1;
  } 
}

impl Renderable for Player {
  fn render(&self) {
    BorderedCircle {
      x: self.pos.x,
      y: self.pos.y,
      radius: PLAYER_RADIUS,
      color: {
        if !self.is_alive() {
          RED
        } else if self.take_hit_cooldown > 0.0 {
          PLAYER_COLOR.with_alpha(
            if (self.take_hit_cooldown * 10.0).round() % 2.0 == 0.0 { 0.5 } else { 1.0 })
        } else {
          PLAYER_COLOR
        }
      },
      b_thick: PLAYER_BORDER_THICKNESS,
      b_color: PLAYER_BORDER_COLOR,
    }.draw();
  }
}

impl Collidable for Player {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: PLAYER_RADIUS }
  }
}
