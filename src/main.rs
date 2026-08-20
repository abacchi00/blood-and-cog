use macroquad::prelude::*;

mod traits;
mod player;
mod direction;
mod bullet;
mod aim;
mod enemy;
mod game_world;
mod game_command;
mod config;
mod wall;
mod floor;
mod arena;

use crate::game_world::GameWorld;
use crate::game_command::GameCommand;

#[macroquad::main("Blood & Cog")]
async fn main() {
  show_mouse(false);

  macroquad::rand::srand(miniquad::date::now() as u64);

  let mut game = GameWorld::new();

  loop {
    let dt = get_frame_time();

    match game.handle_input() {
      GameCommand::Exit => break,
      GameCommand::Continue => {
        game.update(dt);
        game.render();
      }
    }

    next_frame().await;
  }
}
