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
use crate::floor::Floor;
use crate::game_command::GameCommand;

// TODO[refactor]: Create Map struct and impl
pub struct GameWorld {
  player: Player,
  aim: Aim,
  bullets: Vec<Bullet>,
  enemies: Vec<Enemy>,
  walls: Vec<Wall>,
  floors: Vec<Floor>,
  map_grid: Vec<Vec<i32>>,
  map_available_grid_positions: Vec<(usize, usize)>,
}

impl GameWorld {
  pub fn new() -> Self {
    let map_grid: Vec<Vec<i32>> = Vec::from([
      Vec::from([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1]),
      Vec::from([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
    ]);

    let map_initial_player_grid_pos: (usize, usize) = (9, 10);

    let mut map_available_grid_positions: Vec<(usize, usize)> = Vec::new();

    map_grid.iter().enumerate().for_each(|(row_idx, row)| {
      row.iter().enumerate().for_each(|(col_idx, num)| {
        if *num != 1 {
          map_available_grid_positions.push((col_idx, row_idx))
        }
      })
    });


    let mut world = Self {
      player: Player::new(GameWorld::grid_position_to_map_coordinate(map_initial_player_grid_pos)),
      aim: Aim::new(),
      bullets: Vec::new(),
      enemies: Vec::new(),
      walls: Vec::new(),
      floors: Vec::new(),
      map_grid,
      map_available_grid_positions,
    };

    world.spawn_initial_enemies();
    world.spawn_initial_walls();

    world
  }

  fn spawn_initial_enemies(&mut self) {
    for _ in 0..4 {
      self.enemies.push(Enemy::new(self.random_available_map_coordinate()));
    }
  }

  fn spawn_initial_walls(&mut self) {
    self.map_grid.iter().enumerate().for_each(|(row_idx, row)| {
      row.iter().enumerate().for_each(|(col_idx, num)| {
        let x = col_idx as f32 * GRID_CELL_SIZE;
        let y = row_idx as f32 * GRID_CELL_SIZE;

        if *num == 1 {
          self.walls.push(Wall::new(
            Some(GRID_CELL_SIZE),
            Some(GRID_CELL_SIZE),
            Some(x),
            Some(y),
          ));
        } else if *num == 2 {
          self.floors.push(Floor::new(
            Some(GRID_CELL_SIZE),
            Some(GRID_CELL_SIZE),
            Some(x),
            Some(y),
          ));
        }
      });
    });
  }

  pub fn handle_input(&mut self) -> GameCommand {
    if is_key_down(KeyCode::Escape) { return GameCommand::Exit };

    self.player.update_input();

    let screen_mouse_pos = Vec2::from(mouse_position());

    let half_screen = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let offset_from_center = screen_mouse_pos - half_screen;

    let world_mouse_pos = self.player.pos + offset_from_center;

    if is_mouse_button_pressed(MouseButton::Left) {
      self.bullets.push(Bullet::new(self.player.pos, world_mouse_pos));
      self.aim.trigger_click();
    }

    return GameCommand::Continue;
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

  fn grid_position_to_map_coordinate((grid_x, grid_y): (usize, usize)) -> Vec2 {
    return vec2(
      GRID_CELL_SIZE * grid_x as f32 + GRID_CELL_SIZE / 2.0,
      GRID_CELL_SIZE * grid_y as f32 + GRID_CELL_SIZE / 2.0,
    );
  }

  fn random_available_map_coordinate(&self) -> Vec2 {
    let rand_idx = rand::gen_range(0, self.map_available_grid_positions.len());
    let random_position = self.map_available_grid_positions[rand_idx];
    let random_coordinate = GameWorld::grid_position_to_map_coordinate(random_position);
  
    return random_coordinate;
  }

  fn cleanup_and_spawn(&mut self) {
    self.bullets.retain(|b| !b.should_clean());
    self.enemies.retain(|e| !e.should_clean());

    while self.enemies.len() < MIN_ENEMIES_COUNT {
      self.enemies.push(Enemy::new(self.random_available_map_coordinate()));
    }
  }

  pub fn render(&self) {
    let camera = self.get_camera();
    set_camera(&camera);

    clear_background(BG_COLOR);

    for wall in &self.walls { wall.render(); }
    for floor in &self.floors { floor.render(); }
    for bullet in &self.bullets { bullet.render(); }
    for enemy in &self.enemies { enemy.render(); }
    self.player.render();

    set_default_camera();

    // Need to be after set_default_camera
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
