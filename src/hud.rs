use macroquad::prelude::*;

use crate::config::*;
use crate::alt_shapes::BorderedRect;

pub struct Hud {}

impl Hud {
  pub fn render(player_life: f32, player_cogs: i32) {
    let life_bar_w: f32 = screen_width() / 4.0 - HUD_MARGIN;
    let life_bar_x: f32 = HUD_MARGIN;
    let life_bar_y: f32 = HUD_MARGIN;

    BorderedRect {
      x: life_bar_x,
      y: life_bar_y,
      w: life_bar_w,
      h: HUD_LIFE_BAR_HEIGHT,
      color: BLACK,
      b_thick: HUD_LIFE_BAR_BTHICKNESS,
      b_color: WALL_BRICKS_COLOR,
    }.draw();

    draw_text(
      format!("Cogs collected: {}", player_cogs),
      HUD_MARGIN,
      HUD_MARGIN * 2.0 + HUD_LIFE_BAR_HEIGHT + FONT_SIZE_HUD * 0.25,
      FONT_SIZE_HUD,
      MAIN_FONT_COLOR,
    );

    if player_life != 0.0 {
      draw_rectangle(
        life_bar_x + HUD_LIFE_BAR_BTHICKNESS,
        life_bar_y + HUD_LIFE_BAR_BTHICKNESS,
        life_bar_w * player_life / PLAYER_BASE_LIFE - HUD_LIFE_BAR_BTHICKNESS * 2.0,
        HUD_LIFE_BAR_HEIGHT - HUD_LIFE_BAR_BTHICKNESS * 2.0,
        RED,
      );
    } else {
      Self::render_game_over_overlay();
    }
  }
  
  fn render_game_over_overlay() {
    Self::render_opaque_overlay();
    Self::render_centered_text("Game Over!", FONT_SIZE_GAME_OVER, -40.0);
    Self::render_centered_text("Press Esc to exit the game", FONT_SIZE_INSTRUCTIONS, 8.0);
    Self::render_centered_text("Press Space to restart the game", FONT_SIZE_INSTRUCTIONS, 48.0);
  }

  fn render_centered_text(text: &str, font_size: f32, offset_y: f32) {
    let text_dimensions = get_text_center(text, None, font_size as u16, 1.0, 0.0);
    
    draw_text(
      text,
      (screen_width() / 2.0) - text_dimensions.x,
      (screen_height() / 2.0) - text_dimensions.y + offset_y,
      font_size,
      MAIN_FONT_COLOR,
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
