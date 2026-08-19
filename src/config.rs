use macroquad::prelude::*;

pub const BG_COLOR: Color = BLACK;

pub const AIM_COLOR: Color = WHITE;
pub const AIM_BASE_LENGTH: f32 = 10.0;
pub const AIM_BASE_THICKNESS: f32 = 3.0;
pub const AIM_BASE_MOUSE_DISTANCE: f32 = 8.0;
pub const AIM_SCALE_AFTER_CLICK: f32 = 1.6;

pub const BULLET_RADIUS: f32 = 4.0;
pub const BULLET_COLOR: Color = WHITE;
pub const BULLET_VEL: f32 = 1000.0; // pixels-per-second with dt

pub const ENEMY_RADIUS: f32 = 15.0;
pub const ENEMY_BORDER_THICKNESS: f32 = 3.0;
pub const ENEMY_COLOR: Color = Color::new(1.0, 0.5, 0.5, 1.0); // light-red / salmon
pub const ENEMY_INITIAL_LIFE: f32 = 100.0;

pub const MIN_ENEMIES_COUNT: usize = 4;

pub const PLAYER_RADIUS: f32 = 15.0;
pub const PLAYER_BORDER_THICKNESS: f32 = 3.0;
pub const PLAYER_COLOR: Color = WHITE;
pub const PLAYER_BASE_SPEED: f32 = 300.0; // pixels-per-second with dt
