use macroquad::prelude::*;

use crate::config::*;
use crate::alt_shapes::BorderedRect;

pub struct Hud {}

impl Hud {
  pub fn render(player_life: f32) {
    let life_bar_margin: f32 = 24.0;
    let life_bar_w: f32 = screen_width() / 4.0 - life_bar_margin;
    let life_bar_h: f32 = 32.0;
    let life_bar_x: f32 = life_bar_margin;
    let life_bar_y: f32 = life_bar_margin;
    let life_bar_bthick: f32 = 3.0;

    BorderedRect {
      x: life_bar_x,
      y: life_bar_y,
      w: life_bar_w,
      h: life_bar_h,
      color: BLACK,
      b_thick: life_bar_bthick,
      b_color: WALL_BRICKS_COLOR,
    }.draw();

    if player_life != 0.0 {
      draw_rectangle(
        life_bar_x + life_bar_bthick,
        life_bar_y + life_bar_bthick,
        life_bar_w * player_life / PLAYER_BASE_LIFE - life_bar_bthick * 2.0,
        life_bar_h - life_bar_bthick * 2.0,
        RED,
      );
    } else {
      Self::render_game_over_overlay();
    }
  }
  
  fn render_game_over_overlay() {
    Self::render_opaque_overlay();
    Self::render_centered_text("Game Over!", 56.0, -40.0);
    Self::render_centered_text("Press Esc to exit the game", 24.0, 8.0);
    Self::render_centered_text("Press Space to restart the game", 24.0, 48.0);
  }

  fn render_centered_text(text: &str, font_size: f32, offset_y: f32) {
    let text_dimensions = get_text_center(text, None, font_size as u16, 1.0, 0.0);
    
    draw_text(
      text,
      (screen_width() / 2.0) - text_dimensions.x,
      (screen_height() / 2.0) - text_dimensions.y + offset_y,
      font_size,
      WHITE,
    );
  }

  fn render_opaque_overlay() {
    draw_rectangle(
      0.0, 
      0.0, 
      screen_width(), 
      screen_height(), 
      Color::new(0.0, 0.0, 0.0, 0.7) 
    );
  }
}
