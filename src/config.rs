use macroquad::prelude::*;

pub const GRID_CELL_SIZE: f32 = 30.0;

pub const BG_COLOR: Color = Color::from_hex(0x1a1c23);

// Arena
pub const ARENA_MIN_ENEMIES_COUNT: usize = 4;

// Floor
pub const FLOOR_COLOR: Color = Color::from_hex(0x272b38);
pub const FLOOR_DIRT_COLOR: Color = Color::from_hex(0x292f40);

// Wall
pub const WALL_COLOR: Color = Color::from_hex(0x3d4451);
pub const WALL_BRICKS_COLOR: Color = Color::from_hex(0x4d5563);
pub const WALL_SHADOW_COLOR: Color = Color::new(0.0, 0.0, 0.0, 0.4);
pub const WALL_SHADOW_OFFSET_Y: f32 = 8.0;

// Aim
pub const AIM_COLOR: Color = WHITE;
pub const AIM_BASE_LENGTH: f32 = 10.0;
pub const AIM_BASE_THICKNESS: f32 = 3.0;
pub const AIM_BASE_MOUSE_DISTANCE: f32 = 8.0;
pub const AIM_SCALE_AFTER_CLICK: f32 = 1.6;

// Bullet
pub const BULLET_RADIUS: f32 = 4.0;
pub const BULLET_COLOR: Color = WHITE;
pub const BULLET_BASE_SPEED: f32 = 700.0; // pixels-per-second with dt

// Enemy
pub const ENEMY_RADIUS: f32 = GRID_CELL_SIZE / 2.25;
pub const ENEMY_BORDER_THICKNESS: f32 = 2.5;
pub const ENEMY_BORDER_COLOR: Color = Color::from_hex(0x12141a);
pub const ENEMY_INITIAL_LIFE: f32 = 100.0;
pub const ENEMY_BASE_SPEED: f32 = 120.0;
pub const ENEMY_COG_DROP_QUANT_MIN: usize = 1;
pub const ENEMY_COG_DROP_QUANT_MAX: usize = 3;
pub const ENEMY_BODY_COLOR: u32 = 0x5a524a;
pub const ENEMY_METAL_COLOR: u32 = 0xc8c8c8;
pub const ENEMY_CYAN_VISOR: u32 = 0x00e5ff;
pub const ENEMY_WALK_SPEED_MULT: f32 = 12.0;
pub const ENEMY_LEG_COUNT: usize = 3;

// Player
pub const PLAYER_RADIUS: f32 = GRID_CELL_SIZE / 2.0;
pub const PLAYER_BORDER_THICKNESS: f32 = 3.0;
pub const PLAYER_COLOR: Color = Color::from_hex(0xff9933);
pub const PLAYER_BORDER_COLOR: Color = Color::from_hex(0x12141a);
pub const PLAYER_BASE_SPEED: f32 = 300.0; // pixels-per-second with dt
pub const PLAYER_BASE_LIFE: f32 = 100.0;
pub const PLAYER_INVULNERABILITY_DURATION: f32 = 1.5; // seconds

// Player Gun Barrel
pub const PLAYER_BARREL_WIDTH: f32 = 20.0;
pub const PLAYER_BARREL_HEIGHT: f32 = 6.0;
pub const PLAYER_BARREL_BORDER_THICKNESS: f32 = 2.0;

// Cogs
pub const COG_RADIUS: f32 = 5.0;
pub const COG_BORDER_THICKNESS: f32 = 3.0;
pub const COG_SHADOW_OFFSET: f32 = 3.0;
pub const COG_COLOR: Color = LIGHTGRAY;
pub const COG_SHADOW_COLOR: Color = BLACK;

// HUD & Interface
pub const HUD_MARGIN: f32 = 24.0;
pub const HUD_LIFE_BAR_HEIGHT: f32 = 32.0;
pub const HUD_LIFE_BAR_BTHICKNESS: f32 = 3.0;
pub const FONT_SIZE_GAME_OVER: f32 = 56.0;
pub const FONT_SIZE_HUD: f32 = 32.0;
pub const FONT_SIZE_INSTRUCTIONS: f32 = 24.0;
pub const MAIN_FONT_COLOR: Color = WHITE;
pub const HUD_BASE_COG_SCALE: f32 = 2.0;
pub const HUD_COLLECTED_COG_SCALE: f32 = 5.0;
