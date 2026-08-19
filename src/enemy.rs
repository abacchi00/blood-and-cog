use macroquad::prelude::*;

use crate::traits::{
  Renderable,
  Updatable,
  Expirable,
  Collidable, CollisionShape,
};
use crate::config::*;

pub struct Enemy {
  pub pos: Vec2,
  pub radius: f32,
  life: f32,
}

impl Enemy {
  pub fn new(opt_pos: Option<Vec2>) -> Self {
    let pos = opt_pos.unwrap_or_else(|| {
      Vec2::new(
        rand::gen_range(ENEMY_RADIUS, screen_width() - ENEMY_RADIUS),
        rand::gen_range(ENEMY_RADIUS, screen_height() - ENEMY_RADIUS)
      )
    });

    Self {
      pos,
      life: ENEMY_INITIAL_LIFE,
      radius: ENEMY_RADIUS,
    }
  }

  pub fn take_hit(&mut self) {
    self.life -= 20.0;
  }

  pub fn is_alive(&self) -> bool {
    self.life > 0.0
  }
}

impl Renderable for Enemy {
  fn render(&self) {
    draw_circle(self.pos.x, self.pos.y, self.radius, ENEMY_BORDER_COLOR);
    draw_circle(self.pos.x, self.pos.y, self.radius - ENEMY_BORDER_THICKNESS, ENEMY_COLOR);
    draw_text(format!("{}", self.life), self.pos.x - self.radius, self.pos.y + self.radius * 2.0, 20.0, WHITE);
  }
}

impl Updatable for Enemy {
  fn update(&mut self, _dt: f32, _world_width: f32, _world_height: f32) {
    // TODO
  }
}

impl Expirable for Enemy {
  fn should_clean(&self) -> bool {
    !self.is_alive()
  }
}

impl Collidable for Enemy {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: self.radius }
  }
}
