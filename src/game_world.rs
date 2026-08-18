use macroquad::prelude::*;

use crate::traits::Renderable;

use crate::player::Player;
use crate::bullet::Bullet;
use crate::aim::Aim;
use crate::enemy::Enemy;

const MIN_ENEMIES_COUNT: usize = 4;

pub struct GameWorld {
  player: Player,
  aim: Aim,
  bullets: Vec<Bullet>,
  enemies: Vec<Enemy>,
}

impl GameWorld {
  pub fn new() -> Self {
    let mut world = Self {
      player: Player::new(),
      aim: Aim::new(),
      bullets: Vec::new(),
      enemies: Vec::new(),
    };

    world.spawn_initial_enemies();

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

  pub fn handle_input(&mut self) {
    self.player.update_input();

    let mouse_pos = Vec2::from(mouse_position());

    if is_mouse_button_pressed(MouseButton::Left) {
      self.bullets.push(Bullet::new(self.player.pos, mouse_pos));
      self.aim.trigger_click();
    }
  }

  pub fn update(&mut self, dt: f32) {
    self.player.update_pos(dt);
    self.aim.update(dt);
    
    for bullet in &mut self.bullets { bullet.update_pos(dt); }
    for enemy in &mut self.enemies { enemy.update_pos(dt); }

    self.resolve_collisions();
    self.cleanup_and_spawn();
  }

  fn resolve_collisions(&mut self) {
    for bullet in &mut self.bullets {
      for enemy in &mut self.enemies {
        if enemy.is_alive() && bullet.collides_with(enemy.pos, enemy.radius) {
          enemy.take_hit();
          bullet.collided = true;
        }
      }
    }
  }

  fn cleanup_and_spawn(&mut self) {
    // Remove out of bound bullets & bullets that collided with something
    self.bullets.retain(|b| !b.collided && b.is_within_bounds());

    // Remove dead enemies
    self.enemies.retain(|e| e.is_alive());

    // Maintain enemy population density
    while self.enemies.len() < MIN_ENEMIES_COUNT {
      self.enemies.push(Enemy::new(None));
    }
  }

  fn draw_entities<T: Renderable>(entities: &[T]) {
    for entity in entities {
      entity.render();
    }
  }

  pub fn render(&self) {
    clear_background(BLACK);

    Self::draw_entities(&self.bullets);
    Self::draw_entities(&self.enemies);
    self.player.render();
    self.aim.render(); 
  }
}
