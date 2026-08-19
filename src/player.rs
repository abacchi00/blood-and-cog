use macroquad::prelude::*;

use crate::direction::Direction;
use crate::traits::{Renderable, Updatable};
use crate::config::*;

pub struct Player {
  pub dirs: Vec<Direction>,
  pub pos: Vec2,
}

impl Player {
  pub fn new() -> Self {
    Self {
      dirs: Vec::new(),
      pos: Vec2::new(screen_width()/2.0, screen_height()/2.0),
    }
  }

  pub fn update_input(&mut self) {
    self.dirs.clear();
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) { self.dirs.push(Direction::Up); }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) { self.dirs.push(Direction::Down); }
    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) { self.dirs.push(Direction::Left); }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) { self.dirs.push(Direction::Right); }
  }
}

impl Renderable for Player {
  fn render(&self) {
    draw_circle(self.pos.x, self.pos.y, PLAYER_RADIUS, PLAYER_BORDER_COLOR);
    draw_circle(self.pos.x, self.pos.y, PLAYER_RADIUS - PLAYER_BORDER_THICKNESS, PLAYER_COLOR);
  }
}

impl Updatable for Player {
  fn update(&mut self, dt: f32, _world_width: f32, _world_height: f32) {
    let mut dx = 0.0f32;
    let mut dy = 0.0f32;

    for dir in &self.dirs {
      match dir {
        Direction::Up => dy -= 1.0,
        Direction::Down => dy += 1.0,
        Direction::Left => dx -= 1.0,
        Direction::Right => dx += 1.0,
      }
    }

    if dx != 0.0 && dy != 0.0 {
      let normalization = 1.0f32 / (2.0f32).sqrt();
      dx *= normalization;
      dy *= normalization;
    }

    self.pos.x += dx * PLAYER_BASE_SPEED * dt;
    self.pos.y += dy * PLAYER_BASE_SPEED * dt;
  }
}
