use macroquad::prelude::*;

use crate::wall::Wall;
use crate::floor::Floor;
use crate::config::*;
use crate::traits::Renderable;

#[derive(Clone, Copy)]
enum GridCellType {
  W, // Wall
  F, // Floor
  P, // Player
}

use GridCellType::*;

pub struct Arena {
  pub walls: Vec<Wall>,
  pub floors: Vec<Floor>,
  pub available_grid_positions: Vec<(usize, usize)>,
  pub initial_player_pos: Vec2,
}

impl Arena {
  pub fn new() -> Self {
    let grid: Vec<Vec<GridCellType>> = Vec::from([
      Vec::from([W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, W, W, W, W, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, W, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, P, W, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, W, F, F, F, F, F, F, W, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, W, W, W, W, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, W]),
      Vec::from([W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W, W]),
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
          W => walls.push(Wall::new(Some(GRID_CELL_SIZE), Some(GRID_CELL_SIZE), Some(x), Some(y))),
          F => {
            floors.push(Floor::new(Some(GRID_CELL_SIZE), Some(GRID_CELL_SIZE), Some(x), Some(y)));
            available_grid_positions.push((col_idx, row_idx));
          },
          P => {
            floors.push(Floor::new(Some(GRID_CELL_SIZE), Some(GRID_CELL_SIZE), Some(x), Some(y)));
            available_grid_positions.push((col_idx, row_idx));
            initial_player_pos = Arena::grid_to_world_pos((col_idx, row_idx));
          },
        }
      });
    });

    Self {
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

  pub fn render(&self) {
    for wall in &self.walls { wall.render(); }
    for floor in &self.floors { floor.render(); }
  }
}
