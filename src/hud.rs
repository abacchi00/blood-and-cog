use macroquad::prelude::*;

use crate::config::*;
use crate::alt_shapes::BorderedRect;
use crate::cog::Cog;
use crate::traits::*;

pub struct Hud {
  cog_scale: f32,
}

impl Hud {
  pub fn new() -> Self {
    Self { cog_scale: HUD_BASE_COG_SCALE }
  }

  pub fn render(&self, player_life: f32, player_cogs: i32) {
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
      HUD_MARGIN + COG_RADIUS * HUD_BASE_COG_SCALE * 2.0 + HUD_MARGIN * 0.5,
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
    }
    
    Cog::new(vec2(HUD_MARGIN + COG_RADIUS * HUD_BASE_COG_SCALE, HUD_MARGIN * 2.0 + HUD_LIFE_BAR_HEIGHT)).render_scaled(self.cog_scale);

    if player_life == 0.0 {
      Self::render_game_over_overlay();
    }
  }

  pub fn display_cog_collected(&mut self) {
    self.cog_scale = HUD_COLLECTED_COG_SCALE;
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

impl Updatable for Hud {
  fn update(&mut self, dt: f32, _world_width: f32, _world_height: f32) {
    self.cog_scale = self.cog_scale.lerp(HUD_BASE_COG_SCALE, dt * 10.0);
  }
}
