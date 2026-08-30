use macroquad::prelude::*;

use crate::wall::Wall;
use crate::floor::Floor;
use crate::config::*;
use crate::blood_pool::BloodPool;
use crate::traits::{Renderable, RenderableSlice};

#[derive(Clone, Copy)]
enum GridCellType {
  W, // Wall
  F, // Floor
  P, // Player
}

use GridCellType::*;

pub struct Arena {
  grid: Vec<Vec<GridCellType>>,
  pub walls: Vec<Wall>,
  pub floors: Vec<Floor>,
  pub available_grid_positions: Vec<(usize, usize)>,
  pub initial_player_pos: Vec2,
}

impl Arena {
  pub fn new() -> Self {
    let grid: Vec<Vec<GridCellType>> = Vec::from([
      Vec::from([W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W]),
      Vec::from([W, W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W, W]),
      Vec::from([W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W, W, W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, W, W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W, W, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W, F, F, F, W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W, F, F, F, F, F, W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, W, W, F, F, F, F, F, F, F, F, W, W, F, F, F, F, F, F, F, W, W, F, F, F, F, F, F, F, W, W, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, W, F, F, F, F, F, F, F, F, F, F, F, F, F, P, F, F, F, F, F, F, F, F, F, F, F, F, W, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, W, W, F, F, F, F, F, F, F, F, W, W, F, F, F, F, F, F, F, W, W, F, F, F, F, F, F, F, W, W, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W, F, F, F, F, F, W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W, F, F, F, W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, W, W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W, W, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W, W, W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W]),
      Vec::from([W, W, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, W, W]),
      Vec::from([W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W]),
    ]);
    
    let mut available_grid_positions = Vec::new();
    let mut walls = Vec::new();
    let mut floors = Vec::new();

    let mut initial_player_pos: Vec2 =  Default::default();

    grid.iter().enumerate().for_each(|(row_idx, row)| {
      row.iter().enumerate().for_each(|(col_idx, cell_type)| {
        let x = col_idx as f32 * GRID_CELL_SIZE;
        let y = row_idx as f32 * GRID_CELL_SIZE;

        match cell_type {
          W => walls.push(Wall::new(x, y)),
          F => {
            floors.push(Floor::new(x, y));
            available_grid_positions.push((col_idx, row_idx));
          },
          P => {
            floors.push(Floor::new(x, y));
            available_grid_positions.push((col_idx, row_idx));
            initial_player_pos = Arena::grid_to_world_pos((col_idx, row_idx));
          },
        }
      });
    });

    Self {
      grid,
      walls,
      floors,
      available_grid_positions,
      initial_player_pos,
    }
  }

  pub fn grid_to_world_pos((grid_x, grid_y): (usize, usize)) -> Vec2 {
    vec2(
      GRID_CELL_SIZE * grid_x as f32 + GRID_CELL_SIZE / 2.0,
      GRID_CELL_SIZE * grid_y as f32 + GRID_CELL_SIZE / 2.0,
    )
  }

  pub fn random_available_position(&self) -> Vec2 {
    let rand_idx = rand::gen_range(0, self.available_grid_positions.len());
    let pos = self.available_grid_positions[rand_idx];
    Self::grid_to_world_pos(pos)
  }

  // O(1) collision check, using arena grid as advantage
  pub fn is_position_blocked(&self, pos: Vec2, radius: f32) -> bool {
    let check_points = [
      vec2(pos.x - radius, pos.y - radius),
      vec2(pos.x + radius, pos.y - radius),
      vec2(pos.x - radius, pos.y + radius),
      vec2(pos.x + radius, pos.y + radius),
    ];

    for pt in &check_points {
      let col = (pt.x / GRID_CELL_SIZE) as isize;
      let row = (pt.y / GRID_CELL_SIZE) as isize;

      if row >= 0 && row < self.grid.len() as isize && col >= 0 && col < self.grid[0].len() as isize {
        if let GridCellType::W = self.grid[row as usize][col as usize] {
          return true;
        }
      } else {
        return true;
      }
    }

    false
  }

  pub fn render(&self, blood_pools: &Vec<BloodPool>) {
    for floor in &self.floors { floor.render(); }

    blood_pools.render_all();

    for wall in &self.walls { wall.render(); }
  }
}
