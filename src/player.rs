use macroquad::prelude::*;

use crate::config::*;
use crate::traits::{Collidable, CollisionShape, Renderable};
use crate::health::Health;
use crate::impl_damageable;

pub struct Player {
  pub pos: Vec2,
  pub cogs_count: i32,
  pub hp: Health,
  recoil_timer: f32,
}

impl_damageable!(Player);

impl Player {
  pub fn new(pos: Vec2) -> Self {
    Self {
      pos,
      hp: Health::new(PLAYER_BASE_LIFE, PLAYER_INVULNERABILITY_DURATION),
      cogs_count: 0,
      recoil_timer: 0.0,
    }
  }

  pub fn update(&mut self, dt: f32) {
    self.hp.update(dt);
    
    if self.recoil_timer > 0.0 {
      self.recoil_timer -= dt;
      self.recoil_timer = self.recoil_timer.max(0.0);
    }
  }

  pub fn trigger_recoil(&mut self) {
    self.recoil_timer = 0.12;
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

    let forward = vec2(aim_angle.cos(), aim_angle.sin());
    
    let recoil = self.recoil_timer * 12.0;
    self.pos + forward * (PLAYER_RADIUS * 1.1 + 14.0) - forward * recoil
  }
}

impl Collidable for Player {
  fn pos(&self) -> Vec2 { self.pos }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: PLAYER_RADIUS }
  }
}

impl Renderable for Player {
  fn render(&self) {
    let screen_mouse_pos = Vec2::from(mouse_position());
    let half_screen = vec2(screen_width() / 2.0, screen_height() / 2.0);
    
    let mouse_offset = screen_mouse_pos - half_screen;
    let aim_angle = mouse_offset.y.atan2(mouse_offset.x);

    let forward = vec2(aim_angle.cos(), aim_angle.sin());
    let right = vec2(-aim_angle.sin(), aim_angle.cos());
    
    let recoil = self.recoil_timer * 12.0;
    
    let (armor_main, armor_shade) = if !self.hp.is_alive() { 
      (RED, Color::new(0.5, 0.0, 0.0, 1.0)) 
    } else if self.hp.is_injured() && (self.hp.injured_cooldown * 10.0).round() as i32 % 2 == 0 { 
      (Color::new(0.5, 0.5, 0.5, 0.5), Color::new(0.2, 0.2, 0.2, 0.5)) 
    } else { 
      (Color::new(0.25, 0.25, 0.28, 1.0), Color::new(0.12, 0.12, 0.15, 1.0))
    };
    
    let lime_green = Color::new(0.6, 1.0, 0.1, 1.0);
    let dark_green = Color::new(0.1, 0.3, 0.0, 1.0);

    // Kinematics
    let shoulder_span = PLAYER_RADIUS * 0.8;
    let l_shoulder = self.pos - right * shoulder_span;
    let r_shoulder = self.pos + right * shoulder_span;

    let weapon_base = self.pos + forward * (PLAYER_RADIUS * 1.1) - forward * recoil;
    
    let l_hand = weapon_base + forward * 8.0 - right * 2.5; 
    let r_hand = weapon_base - forward * 2.0 + right * 2.5; 

    let l_elbow = self.pos + forward * (PLAYER_RADIUS * 0.6) - right * (PLAYER_RADIUS * 1.25);
    let r_elbow = self.pos + forward * (PLAYER_RADIUS * 0.2) + right * (PLAYER_RADIUS * 1.25);

    // Shadow
    draw_circle(self.pos.x, self.pos.y + 3.0, PLAYER_RADIUS * 1.25, Color::new(0.0, 0.0, 0.0, 0.4));

    // Arms
    let arm_shade = 10.0;
    let arm_main = 6.0;

    draw_line(l_shoulder.x, l_shoulder.y, l_elbow.x, l_elbow.y, arm_shade, armor_shade);
    draw_line(l_elbow.x, l_elbow.y, l_hand.x, l_hand.y, arm_shade, armor_shade);
    draw_line(r_shoulder.x, r_shoulder.y, r_elbow.x, r_elbow.y, arm_shade, armor_shade);
    draw_line(r_elbow.x, r_elbow.y, r_hand.x, r_hand.y, arm_shade, armor_shade);
    draw_circle(l_elbow.x, l_elbow.y, arm_shade / 2.0, armor_shade);
    draw_circle(r_elbow.x, r_elbow.y, arm_shade / 2.0, armor_shade);

    draw_line(l_shoulder.x, l_shoulder.y, l_elbow.x, l_elbow.y, arm_main, armor_main);
    draw_line(l_elbow.x, l_elbow.y, l_hand.x, l_hand.y, arm_main, armor_main);
    draw_line(r_shoulder.x, r_shoulder.y, r_elbow.x, r_elbow.y, arm_main, armor_main);
    draw_line(r_elbow.x, r_elbow.y, r_hand.x, r_hand.y, arm_main, armor_main);
    draw_circle(l_elbow.x, l_elbow.y, arm_main / 2.0, armor_main);
    draw_circle(r_elbow.x, r_elbow.y, arm_main / 2.0, armor_main);

    // Torso
    let torso_thick = PLAYER_RADIUS * 1.35;
    draw_line(l_shoulder.x, l_shoulder.y, r_shoulder.x, r_shoulder.y, torso_thick, armor_shade);
    draw_circle(l_shoulder.x, l_shoulder.y, torso_thick / 2.0, armor_shade);
    draw_circle(r_shoulder.x, r_shoulder.y, torso_thick / 2.0, armor_shade);

    let inner_thick = torso_thick - 3.0;
    draw_line(l_shoulder.x, l_shoulder.y, r_shoulder.x, r_shoulder.y, inner_thick, armor_main);
    draw_circle(l_shoulder.x, l_shoulder.y, inner_thick / 2.0, armor_main);
    draw_circle(r_shoulder.x, r_shoulder.y, inner_thick / 2.0, armor_main);

    // Hands 
    let glove_color = Color::new(0.12, 0.12, 0.12, 1.0);
    draw_circle(l_hand.x, l_hand.y, 4.5, armor_shade);
    draw_circle(l_hand.x, l_hand.y, 3.0, glove_color);
    draw_circle(r_hand.x, r_hand.y, 4.5, armor_shade);
    draw_circle(r_hand.x, r_hand.y, 3.0, glove_color);

    // Weapon 
    let stock = weapon_base - forward * 5.0;
    let muzzle = weapon_base + forward * 14.0;
    
    draw_line(stock.x, stock.y, muzzle.x, muzzle.y, 7.0, Color::new(0.1, 0.1, 0.1, 1.0));
    draw_line(stock.x, stock.y, muzzle.x, muzzle.y, 3.5, LIGHTGRAY);
    
    let mag_start = weapon_base + forward * 3.0;
    let mag_end = mag_start + right * 6.0;
    draw_line(mag_start.x, mag_start.y, mag_end.x, mag_end.y, 5.0, Color::new(0.1, 0.1, 0.1, 1.0));
    draw_line(mag_start.x, mag_start.y, mag_end.x, mag_end.y, 2.5, DARKGRAY);

    // Head
    let head_radius = PLAYER_RADIUS * 0.75;
    draw_circle(self.pos.x, self.pos.y, head_radius, armor_shade);
    draw_circle(self.pos.x, self.pos.y, head_radius - 2.0, armor_main);

    // Visor
    let visor_center = self.pos + forward * (head_radius * 0.3);
    let mask_center = self.pos - forward * (head_radius * 0.05);
    
    draw_circle(visor_center.x, visor_center.y, head_radius * 0.85, dark_green);
    draw_circle(visor_center.x, visor_center.y, head_radius * 0.70, lime_green);
    draw_circle(mask_center.x, mask_center.y, head_radius * 0.85, armor_main);

    // Muzzle Flash
    // O recoil takes only 0.12s
    if self.recoil_timer > 0.06 {
      let flash_pos = muzzle + forward * 4.0;
      let flash_scale = (self.recoil_timer - 0.06) * 150.0; 
      
      draw_circle(flash_pos.x, flash_pos.y, flash_scale, Color::new(1.0, 0.6, 0.0, 0.6));
      draw_circle(flash_pos.x, flash_pos.y, flash_scale * 0.5, Color::new(1.0, 1.0, 0.8, 0.9));
    }
  }
}
