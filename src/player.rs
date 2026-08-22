use macroquad::prelude::*;

use crate::config::*;
use crate::traits::{Collidable, CollisionShape, Renderable};
use crate::alt_shapes::BorderedCircle;

pub struct Player {
  pub pos: Vec2,
  pub life: f32,
  take_hit_cooldown: f32,
  pub cogs_count: i32,
}

impl Player {
  pub fn new(pos: Vec2) -> Self {
    Self {
      pos,
      life: PLAYER_BASE_LIFE,
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

  pub fn get_barrel_tip_pos(&self) -> Vec2 {
    let screen_mouse_pos = Vec2::from(mouse_position());
    let half_screen = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let mouse_offset = screen_mouse_pos - half_screen;
    let aim_angle = mouse_offset.y.atan2(mouse_offset.x);

    let total_length = PLAYER_BARREL_WIDTH + PLAYER_BARREL_BORDER_THICKNESS;
    
    vec2(
      self.pos.x + aim_angle.cos() * total_length,
      self.pos.y + aim_angle.sin() * total_length,
    )
  }
}

impl Renderable for Player {
  fn render(&self) {
    let screen_mouse_pos = Vec2::from(mouse_position());
    let half_screen = vec2(screen_width() / 2.0, screen_height() / 2.0);
    
    let mouse_offset = screen_mouse_pos - half_screen;
    let aim_angle = mouse_offset.y.atan2(mouse_offset.x);

    let base_params = DrawRectangleParams {
      offset: vec2(0.0, 0.5),
      rotation: aim_angle,
      color: PLAYER_BORDER_COLOR,
    };

    draw_rectangle_ex(
      self.pos.x - aim_angle.cos() * PLAYER_BARREL_BORDER_THICKNESS,
      self.pos.y - aim_angle.sin() * PLAYER_BARREL_BORDER_THICKNESS,
      PLAYER_BARREL_WIDTH + PLAYER_BARREL_BORDER_THICKNESS * 2.0,
      PLAYER_BARREL_HEIGHT + PLAYER_BARREL_BORDER_THICKNESS * 2.0,
      base_params,
    );

    draw_rectangle_ex(
      self.pos.x,
      self.pos.y,
      PLAYER_BARREL_WIDTH,
      PLAYER_BARREL_HEIGHT,
      DrawRectangleParams {
        offset: vec2(0.0, 0.5),
        rotation: aim_angle,
        color: COG_COLOR,
      },
    );

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

    draw_circle(self.pos.x, self.pos.y, 4.0, PLAYER_BORDER_COLOR);
  }
}

impl Collidable for Player {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: PLAYER_RADIUS }
  }
}
