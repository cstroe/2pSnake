use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;

#[derive(Resource, Debug, Clone)]
pub struct ArenaLayout {
    pub cell_size: f32,
    pub grid_origin: Vec2, // Bottom-left coordinate of cell (0, 0)
    pub grid_center: Vec2,
    pub window_size: Vec2,
}

impl Default for ArenaLayout {
    fn default() -> Self {
        Self {
            cell_size: 0.0,
            grid_origin: Vec2::ZERO,
            grid_center: Vec2::ZERO,
            window_size: Vec2::ZERO,
        }
    }
}

impl ArenaLayout {
    /// Convert grid coordinate (0..112, 0..70) to 2D world space coordinates
    pub fn grid_to_world(&self, cell: IVec2) -> Vec2 {
        self.grid_origin + Vec2::new(
            (cell.x as f32 + 0.5) * self.cell_size,
            (cell.y as f32 + 0.5) * self.cell_size,
        )
    }

    /// Recalculate cell size and origin to maximize screen usage within 16:9 aspect
    pub fn update(&mut self, window_w: f32, window_h: f32) {
        self.window_size = Vec2::new(window_w, window_h);

        let available_w = (window_w - ARENA_PADDING_PX * 2.0).max(100.0);
        let available_h = (window_h - HUD_HEIGHT_PX - ARENA_PADDING_PX * 2.0).max(100.0);

        let scale_w = available_w / GRID_WIDTH as f32;
        let scale_h = available_h / GRID_HEIGHT as f32;

        self.cell_size = scale_w.min(scale_h).max(0.5);

        let total_arena_w = GRID_WIDTH as f32 * self.cell_size;
        let total_arena_h = GRID_HEIGHT as f32 * self.cell_size;

        // Position grid centered horizontally, and below the HUD bar
        let center_y = -HUD_HEIGHT_PX / 2.0;
        self.grid_center = Vec2::new(0.0, center_y);
        self.grid_origin = Vec2::new(-total_arena_w / 2.0, center_y - total_arena_h / 2.0);
    }
}

#[derive(Resource)]
pub struct GameClock {
    pub tick_timer: Timer,
    pub tick_duration: f32,
    pub round_time_remaining: f32,
}

impl Default for GameClock {
    fn default() -> Self {
        Self {
            tick_timer: Timer::from_seconds(DEFAULT_TICK_SECONDS, TimerMode::Repeating),
            tick_duration: DEFAULT_TICK_SECONDS,
            round_time_remaining: INITIAL_ROUND_TIME_SECS,
        }
    }
}

impl GameClock {
    pub fn adjust_speed(&mut self, delta: f32) {
        self.tick_duration = (self.tick_duration + delta).clamp(MIN_TICK_SECONDS, MAX_TICK_SECONDS);
        self.tick_timer.set_duration(std::time::Duration::from_secs_f32(self.tick_duration));
    }

    pub fn reset(&mut self) {
        self.tick_duration = DEFAULT_TICK_SECONDS;
        self.tick_timer = Timer::from_seconds(DEFAULT_TICK_SECONDS, TimerMode::Repeating);
        self.round_time_remaining = INITIAL_ROUND_TIME_SECS;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountdownStep {
    Three,
    Two,
    One,
    Start,
    Done,
}

#[derive(Resource)]
pub struct CountdownState {
    pub timer: Timer,
    pub step: CountdownStep,
}

impl Default for CountdownState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            step: CountdownStep::Three,
        }
    }
}

#[derive(Resource, Default)]
pub struct ControllerAssignments {
    pub p1_device: Option<ControllerDevice>,
    pub p1_ready: bool,
    pub p2_device: Option<ControllerDevice>,
    pub p2_ready: bool,
}

impl ControllerAssignments {
    pub fn is_ready_to_start(&self) -> bool {
        self.p1_device.is_some() && self.p1_ready && self.p2_device.is_some() && self.p2_ready
    }

    pub fn reset_ready(&mut self) {
        self.p1_ready = false;
        self.p2_ready = false;
    }
}

#[derive(Resource, Default)]
pub struct GameWinner {
    pub winner: Option<PlayerId>,
    pub reason: String,
    pub p1_final_score: u32,
    pub p2_final_score: u32,
}

#[derive(Resource)]
pub struct NotificationMessage {
    pub text: String,
    pub timer: Timer,
}

impl Default for NotificationMessage {
    fn default() -> Self {
        Self {
            text: "Collect fruits & box in your opponent!".to_string(),
            timer: Timer::from_seconds(4.0, TimerMode::Once),
        }
    }
}

pub use crate::audio::SoundAssets;

