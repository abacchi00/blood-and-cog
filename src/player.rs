use macroquad::prelude::*;
use crate::direction::Direction;

const PLAYER_RADIUS: f32 = 15.0;
const PLAYER_BORDER_THICKNESS: f32 = 3.0;
const PLAYER_COLOR: Color = WHITE;
const BG_COLOR: Color = BLACK; // todo: move this to global consts
const PLAYER_BASE_SPEED: f32 = 300.0; // pixels-per-second with dt

pub struct Player {
  pub dirs: Vec<Direction>,
  pub pos: Vec2,
}

impl Player {
  pub fn new() -> Self {
    Self {
      dirs: Vec::new(),
      pos: Vec2::new(50.0, 50.0),
    }
  }

  pub fn change_dirs(&mut self, new_dirs: Vec<Direction>) {
    self.dirs = new_dirs;
  }

  pub fn update_pos(&mut self, dt: f32) {
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

  pub fn render(&self) {
    draw_circle(self.pos.x, self.pos.y, PLAYER_RADIUS, PLAYER_COLOR);
    draw_circle(self.pos.x, self.pos.y, PLAYER_RADIUS - PLAYER_BORDER_THICKNESS, BG_COLOR);
  }
}
