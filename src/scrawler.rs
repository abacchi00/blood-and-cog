use macroquad::prelude::*;

use crate::alt_shapes::BorderedCircle;
use crate::config::*;
use crate::traits::{Collidable, CollisionShape, Expirable, Renderable};

pub struct Scrawler {
  pub pos: Vec2,
  pub radius: f32,
  life: f32,
  facing_angle: f32,
  walk_anim_timer: f32,
  injured_cooldown: f32,
}

impl Scrawler {
  pub fn new(pos: Vec2) -> Self {
    Self {
      pos,
      radius: SCRAWLER_RADIUS,
      life: SCRAWLER_INITIAL_LIFE,
      facing_angle: 0.0,
      walk_anim_timer: 0.0,
      injured_cooldown: 0.0,
    }
  }

  pub fn update(&mut self, player_pos: Vec2, dt: f32) -> Vec2 {
    if self.injured_cooldown > 0.0 {
      self.injured_cooldown -= dt;
      self.injured_cooldown = self.injured_cooldown.max(0.0);
    }

    let direction = player_pos - self.pos;
    if direction.length_squared() > 0.001 {
      self.facing_angle = direction.y.atan2(direction.x);
      self.walk_anim_timer += dt * SCRAWLER_WALK_SPEED_MULT;
    }

    let normalized_dir = direction.normalize_or_zero();
    normalized_dir * SCRAWLER_BASE_SPEED * dt
  }

  pub fn take_hit(&mut self) {
    if self.injured_cooldown <= 0.0 {
      self.injured_cooldown = SCRAWLER_INJURED_DURATION; // seconds
    }

    self.life -= 20.0;
    self.life = self.life.max(0.0);
  }

  pub fn is_alive(&self) -> bool {
    self.life > 0.0
  }

  fn get_color(&self, color: Color) -> Color {
    if self.injured_cooldown > 0.0 {
      return SCRAWLER_INJURED_COLOR;
    }

    color
  }
}

impl Collidable for Scrawler {
  fn pos(&self) -> Vec2 {
    self.pos
  }
  fn shape(&self) -> CollisionShape {
    CollisionShape::Circle { radius: self.radius }
  }
}

impl Expirable for Scrawler {
  fn should_clean(&self) -> bool {
    !self.is_alive()
  }
}

impl Renderable for Scrawler {
  fn render(&self) {
    let radius = self.radius;

    // Main body projected shadow on the floor (Y-axis offset)
    draw_circle(
      self.pos.x,
      self.pos.y + 4.0,
      radius * 0.9,
      Color::new(0.0, 0.0, 0.0, 0.4),
    );

    // Procedural mechanical spider legs with triangular claws
    let metal_color = self.get_color(SCRAWLER_METAL_COLOR);

    for i in 0..SCRAWLER_LEG_COUNT {
      for side in [-1.0, 1.0] {
        let base_angle_offset = side * (0.8 + (i as f32) * 0.85);
        let leg_angle = self.facing_angle + base_angle_offset;

        // Mechanical hydraulic walk cycle
        let phase = self.walk_anim_timer + (i as f32) * 1.5 + if side > 0.0 { 0.0 } else { std::f32::consts::PI };
        let raw_wave = phase.sin();
        let stride = raw_wave * raw_wave.abs() * 5.0;

        let hip_x = self.pos.x + leg_angle.cos() * (radius * 0.6);
        let hip_y = self.pos.y + leg_angle.sin() * (radius * 0.6);

        let foot_distance = radius * 1.3 + stride;
        let foot_x = self.pos.x + leg_angle.cos() * foot_distance;
        let foot_y = self.pos.y + leg_angle.sin() * foot_distance;

        let knee_x = (hip_x + foot_x) * 0.5 + side * leg_angle.sin() * 7.0;
        let knee_y = (hip_y + foot_y) * 0.5 - side * leg_angle.cos() * 7.0;

        // Leg shadow (Y-axis offset)
        let shadow_y = 2.0;
        let shadow_color = Color::new(0.0, 0.0, 0.0, 0.3);

        draw_line(hip_x, hip_y + shadow_y, knee_x, knee_y + shadow_y, 3.0, shadow_color);
        draw_line(knee_x, knee_y + shadow_y, foot_x, foot_y + shadow_y, 3.0, shadow_color);

        // Actual leg segments
        draw_line(hip_x, hip_y, knee_x, knee_y, 3.0, self.get_color(SCRAWLER_BORDER_COLOR));
        draw_line(knee_x, knee_y, foot_x, foot_y, 3.0, self.get_color(SCRAWLER_BORDER_COLOR));
        draw_line(hip_x, hip_y, knee_x, knee_y, 1.5, metal_color);
        draw_line(knee_x, knee_y, foot_x, foot_y, 1.0, metal_color);

        // Triangular claw foot anchored to the ground
        let claw_tip_x = foot_x + leg_angle.cos() * 4.0;
        let claw_tip_y = foot_y + leg_angle.sin() * 4.0;

        let perp_x = -leg_angle.sin() * 1.5;
        let perp_y = leg_angle.cos() * 1.5;

        draw_triangle(
          vec2(foot_x + perp_x, foot_y + perp_y),
          vec2(foot_x - perp_x, foot_y - perp_y),
          vec2(claw_tip_x, claw_tip_y),
          metal_color,
        );
        draw_line(foot_x + perp_x, foot_y + perp_y, claw_tip_x, claw_tip_y, 1.0, self.get_color(DARKGRAY));
        draw_line(foot_x - perp_x, foot_y - perp_y, claw_tip_x, claw_tip_y, 1.0, self.get_color(DARKGRAY));
      }
    }

    // Main body (industrial metallic gray with border)
    BorderedCircle {
      x: self.pos.x,
      y: self.pos.y,
      radius,
      color: self.get_color(SCRAWLER_BODY_COLOR),
      b_thick: SCRAWLER_BORDER_THICKNESS,
      b_color: self.get_color(SCRAWLER_BORDER_COLOR),
    }.draw();

    // Internal industrial rivets rotating with facing angle
    for i in 0..4 {
      let angle = (i as f32) * std::f32::consts::FRAC_PI_2 + self.facing_angle;
      let rivet_x = self.pos.x + angle.cos() * (radius * 0.55);
      let rivet_y = self.pos.y + angle.sin() * (radius * 0.55);

      draw_circle(rivet_x, rivet_y, 2.0, self.get_color(SCRAWLER_BORDER_COLOR));
      draw_circle(rivet_x, rivet_y, 1.5, self.get_color(metal_color));
    }

    // Tactical cyan visor pointing toward the player
    let visor_distance = radius * 0.6;
    let visor_x = self.pos.x + self.facing_angle.cos() * visor_distance;
    let visor_y = self.pos.y + self.facing_angle.sin() * visor_distance;

    draw_rectangle_ex(
      visor_x,
      visor_y,
      10.0,
      6.0,
      DrawRectangleParams {
        offset: vec2(0.5, 0.5),
        rotation: self.facing_angle,
        color: self.get_color(SCRAWLER_BORDER_COLOR),
      },
    );
    draw_rectangle_ex(
      visor_x,
      visor_y,
      6.0,
      3.0,
      DrawRectangleParams {
        offset: vec2(0.5, 0.5),
        rotation: self.facing_angle,
        color: self.get_color(SCRAWLER_VISOR_COLOR),
      },
    );
  }
}
