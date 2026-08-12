use macroquad::prelude::*;

#[macroquad::main("Shooter")]
async fn main() {
  println!("Hello, world!");
  let mut x = 50.0;
  let mut y = 50.0;
  let mut direction_x = 1.0;
  let mut direction_y = 1.0;
  let radius = 15.0;
  let x_increment = 5.0;
  let y_increment = 5.0;

  loop {
    let sw = screen_width();
    let sh = screen_height();

    x += x_increment * direction_x;
    y += y_increment * direction_y;

    if x > sw - radius { direction_x = -1.0; }
    else if x < radius { direction_x = 1.0; }

    if y > sh - radius { direction_y = -1.0; }
    else if y < radius{ direction_y = 1.0; }

    draw_circle(x, y, radius, RED);

    next_frame().await;
  }
}
