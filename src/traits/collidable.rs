use macroquad::prelude::Vec2;

pub enum CollisionShape {
  Circle { radius: f32 },
  Rectangle { w: f32, h: f32 }, 
}

pub trait Collidable {
  fn pos(&self) -> Vec2;
  fn shape(&self) -> CollisionShape;
}

pub fn check_collision(pos_a: Vec2, shape_a: CollisionShape, pos_b: Vec2, shape_b: CollisionShape) -> bool {
  match (shape_a, shape_b) {
    (CollisionShape::Circle { radius: r1 }, CollisionShape::Circle { radius: r2 }) => {
      let dx = pos_a.x - pos_b.x;
      let dy = pos_a.y - pos_b.y;
      let c_sqrd = (dx * dx) + (dy * dy);
      let radius_sum = r1 + r2;
      c_sqrd < (radius_sum * radius_sum)
    }

    (CollisionShape::Circle { radius: r }, CollisionShape::Rectangle { w, h }) => {
      circle_to_rect(pos_a, r, pos_b, w, h)
    }

    (CollisionShape::Rectangle { w, h }, CollisionShape::Circle { radius: r }) => {
      circle_to_rect(pos_b, r, pos_a, w, h)
    }

    // TODO: fix
    (CollisionShape::Rectangle { w: w1, h: h1 }, CollisionShape::Rectangle { w: w2, h: h2 }) => {
      let a_min_x = pos_a.x - w1 / 2.0;
      let a_max_x = pos_a.x + w1 / 2.0;
      let a_min_y = pos_a.y - h1 / 2.0;
      let a_max_y = pos_a.y + h1 / 2.0;

      let b_min_x = pos_b.x - w2 / 2.0;
      let b_max_x = pos_b.x + w2 / 2.0;
      let b_min_y = pos_b.y - h2 / 2.0;
      let b_max_y = pos_b.y + h2 / 2.0;

      a_min_x < b_max_x && a_max_x > b_min_x && a_min_y < b_max_y && a_max_y > b_min_y
    }
  }
}

fn circle_to_rect(c_pos: Vec2, r: f32, rect_pos: Vec2, w: f32, h: f32) -> bool {
  let closest_x = c_pos.x.clamp(rect_pos.x, rect_pos.x + w);
  let closest_y = c_pos.y.clamp(rect_pos.y, rect_pos.y + h);

  let dx = c_pos.x - closest_x;
  let dy = c_pos.y - closest_y;
  let distance_squared = (dx * dx) + (dy * dy);

  distance_squared < (r * r)
}
