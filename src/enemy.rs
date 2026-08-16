use macroquad::prelude::*;

const ENEMY_RADIUS: f32 = 15.0;
const ENEMY_BORDER_THICKNESS: f32 = 3.0;
const ENEMY_COLOR: Color = Color::new(1.0, 0.5, 0.5, 1.0); // light-red / salmon
// const ENEMY_INITIAL_LIFE: f32 = 100.0;
const BG_COLOR: Color = BLACK; // todo: move this to global consts

pub struct Enemy {
  pos: Vec2,
  // life: f32,
}

impl Enemy {
  pub fn new(pos: Vec2) -> Self {
    Self {
      pos,
      // life: ENEMY_INITIAL_LIFE,
    }
  }

  // pub fn take_hit(&mut self, damage: f32) {
  //   self.life -= damage;
  // }

  pub fn render(&self) {
    draw_circle(self.pos.x, self.pos.y, ENEMY_RADIUS, ENEMY_COLOR);
    draw_circle(self.pos.x, self.pos.y, ENEMY_RADIUS - ENEMY_BORDER_THICKNESS, BG_COLOR);
  }
}
