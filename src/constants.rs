use bevy::prelude::*;

pub const GRID_WIDTH: i32 = 100;
pub const GRID_HEIGHT: i32 = 62;

pub const INITIAL_SNAKE_LENGTH: usize = 6;
pub const DEFAULT_TICK_SECONDS: f32 = 0.2; // 200 milliseconds per tick
pub const MIN_TICK_SECONDS: f32 = 0.02;
pub const MAX_TICK_SECONDS: f32 = 2.0;
pub const TICK_STEP: f32 = 0.02;

pub const INITIAL_ROUND_TIME_SECS: f32 = 300.0; // 5 minutes
pub const NUM_CONCURRENT_FRUITS: usize = 24;

pub const HUD_HEIGHT_PX: f32 = 76.0;
pub const ARENA_PADDING_PX: f32 = 16.0;

// Player 1 theme (Cyan / Neon Blue)
pub const COLOR_P1: Color = Color::srgb(0.0, 0.9, 1.0);
#[allow(dead_code)]
pub const COLOR_P1_HEAD: Color = Color::srgb(0.7, 1.0, 1.0);
#[allow(dead_code)]
pub const COLOR_P1_EYES: Color = Color::srgb(1.0, 1.0, 1.0);

// Player 2 theme (Orange / Neon Coral)
pub const COLOR_P2: Color = Color::srgb(1.0, 0.4, 0.05);
#[allow(dead_code)]
pub const COLOR_P2_HEAD: Color = Color::srgb(1.0, 0.8, 0.4);
#[allow(dead_code)]
pub const COLOR_P2_EYES: Color = Color::srgb(1.0, 1.0, 1.0);

// Arena & UI theme
pub const COLOR_BG: Color = Color::srgb(0.03, 0.04, 0.08);
pub const COLOR_ARENA_BG: Color = Color::srgb(0.05, 0.07, 0.12);
pub const COLOR_ARENA_BORDER: Color = Color::srgb(0.15, 0.35, 0.55);
pub const COLOR_HUD_BG: Color = Color::srgba(0.05, 0.08, 0.14, 0.95);
pub const COLOR_PANEL_BG: Color = Color::srgba(0.08, 0.12, 0.20, 0.95);
pub const COLOR_READY_GREEN: Color = Color::srgb(0.1, 0.9, 0.3);
pub const COLOR_TEXT_MUTED: Color = Color::srgb(0.55, 0.65, 0.75);
