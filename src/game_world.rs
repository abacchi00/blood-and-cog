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
    for _ in 0..4 { self.walls.push(Wall::new()) }
  }

  pub fn handle_input(&mut self) {
    self.player.update_input();

    let mouse_pos = Vec2::from(mouse_position());

    if is_mouse_button_pressed(MouseButton::Left) {
      self.bullets.push(Bullet::new(self.player.pos, mouse_pos));
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
    clear_background(BLACK);

    for bullet in &self.bullets { bullet.render(); }
    for enemy in &self.enemies { enemy.render(); }
    for wall in &self.walls { wall.render(); }
    self.player.render();
    self.aim.render(); 
  }
}
