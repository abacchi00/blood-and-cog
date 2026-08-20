use macroquad::prelude::*;

use crate::traits::{
  Renderable,
  Updatable,
  Expirable,

  Collidable, check_collision,
};
use crate::config::*;

use crate::player::Player;
use crate::bullet::Bullet;
use crate::aim::Aim;
use crate::enemy::Enemy;
use crate::wall::Wall;

pub struct GameWorld {
  player: Player,
  aim: Aim,
  bullets: Vec<Bullet>,
  enemies: Vec<Enemy>,
  walls: Vec<Wall>,
}

impl GameWorld {
  pub fn new() -> Self {
    let mut world = Self {
      player: Player::new(),
      aim: Aim::new(),
      bullets: Vec::new(),
      enemies: Vec::new(),
      walls: Vec::new(),
    };

    world.spawn_initial_enemies();
    world.spawn_initial_walls();

    world
  }

  fn spawn_initial_enemies(&mut self) {
    let w = screen_width() / 3.0;
    let h = screen_height() / 3.0;
    let p = self.player.pos;
    
    self.enemies.push(Enemy::new(Some(Vec2::new(p.x - w, p.y - h))));
    self.enemies.push(Enemy::new(Some(Vec2::new(p.x - w, p.y + h))));
    self.enemies.push(Enemy::new(Some(Vec2::new(p.x + w, p.y - h))));
    self.enemies.push(Enemy::new(Some(Vec2::new(p.x + w, p.y + h))));
  }

  fn spawn_initial_walls(&mut self) {
    // TODO: make it a const, or extract to other file
    let wall_map: Vec<Vec<i32>> = Vec::from([
      Vec::from([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]),
      Vec::from([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
    ]);

    let wall_w = screen_width() / wall_map.len() as f32;
    let wall_h = screen_height() / wall_map[0].len() as f32; 

    wall_map.iter().enumerate().for_each(|(row_idx, row)| {
      row.iter().enumerate().for_each(|(col_idx, num)| {
        if *num == 1 {
          let x = col_idx as f32 * wall_w;
          let y = row_idx as f32 * wall_h;

          self.walls.push(Wall::new(
            Some(wall_w),
            Some(wall_h),
            Some(x),
            Some(y),
          ));
        }
      });
    });
  }

  pub fn handle_input(&mut self) {
    self.player.update_input();

    let screen_mouse_pos = Vec2::from(mouse_position());

    let half_screen = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let offset_from_center = screen_mouse_pos - half_screen;

    let world_mouse_pos = self.player.pos + offset_from_center;

    if is_mouse_button_pressed(MouseButton::Left) {
      self.bullets.push(Bullet::new(self.player.pos, world_mouse_pos));
      self.aim.trigger_click();
    }
  }

  pub fn update(&mut self, dt: f32) {
    let sw = screen_width();
    let sh = screen_height();

    self.player.update(dt, sw, sh);
    self.aim.update(dt, sw, sh);
    
    for bullet in &mut self.bullets { bullet.update(dt, sw, sh); }
    for enemy in &mut self.enemies { enemy.update(dt, sw, sh); }

    self.resolve_collisions();
    self.cleanup_and_spawn();
  }

  fn resolve_collisions(&mut self) {
    for bullet in &mut self.bullets {
      for enemy in &mut self.enemies {
        if enemy.is_alive() && check_collision(bullet.pos(), bullet.shape(), enemy.pos(), enemy.shape()) {
          enemy.take_hit();
          bullet.collided = true;
        }
      }

      for wall in &mut self.walls {
        if check_collision(bullet.pos(), bullet.shape(), wall.pos(), wall.shape()) {
          bullet.collided = true;
        }
      }
    }
  }

  fn cleanup_and_spawn(&mut self) {
    self.bullets.retain(|b| !b.should_clean());
    self.enemies.retain(|e| !e.should_clean());

    while self.enemies.len() < MIN_ENEMIES_COUNT {
      self.enemies.push(Enemy::new(None));
    }
  }

  pub fn render(&self) {
    let camera = self.get_camera();
    set_camera(&camera);

    clear_background(BG_COLOR);

    for bullet in &self.bullets { bullet.render(); }
    for enemy in &self.enemies { enemy.render(); }
    for wall in &self.walls { wall.render(); }
    self.player.render();
    
    set_default_camera();

    self.aim.render(); 
  }

  fn get_camera(&self) -> Camera2D {
    Camera2D {
      target: self.player.pos,
      zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
      ..Default::default()
    }
  }
}
