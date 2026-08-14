use macroquad::prelude::*;

mod player;
mod direction;
mod position;

use player::Player;
use direction::Direction;

#[derive(Clone, Copy, PartialEq)]
pub struct Pos {
  pub x: f32,
  pub y: f32,
}

#[macroquad::main("Shooter")]
async fn main() {
  let mut player = Player::new();
  let radius = 15.0;

  loop {
    player.change_dirs(Direction::current_directions());
    player.update_pos();

    clear_background(BLACK);
    draw_circle(player.pos.x, player.pos.y, radius, RED);

    next_frame().await;
  }
}
