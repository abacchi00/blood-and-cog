use macroquad::prelude::*;

use crate::config::*;

pub struct Hud {}

impl Hud {
  pub fn render(player_life: f32) {
    let text =
      if player_life == 0.0 { "Game over :(".to_string() }
      else { format!("Life: {} / {}", player_life, PLAYER_BASE_LIFE) };

    draw_text(&text, 48.0, screen_height() - 48.0, 48.0, WHITE);
  }
}
