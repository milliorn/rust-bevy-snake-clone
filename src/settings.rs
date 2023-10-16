use bevy::prelude::*;

// Constants for grid and window dimensions
pub const CELL_SIZE: f32 = 80.0;
pub const COLS: i32 = 10;
pub const ROWS: i32 = 10;
pub const WINDOW_HEIGHT: f32 = (CELL_SIZE * ROWS as f32);
pub const WINDOW_WIDTH: f32 = (CELL_SIZE * COLS as f32);
pub const GAME_TITLE: &str = "Snake";

// Initial snake parameters
pub const START_LENGTH: i32 = 3;
pub const START_ROW: i32 = ROWS / 2;
pub const START_COL: i32 = START_LENGTH + 2;

// pub Constants for shadow rendering
pub const SHADOW_COLOR: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);
pub const SHADOW_SIZE: Vec2 = Vec2::new(4.0, 4.0);
