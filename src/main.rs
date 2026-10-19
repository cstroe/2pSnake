mod constants;
mod types;
mod resources;
mod components;
mod grid;
mod controller_select;
mod countdown;
mod snake;
mod fruits;
mod input;
mod game_tick;
mod hud;
mod round_over;

use bevy::prelude::*;
use constants::*;
use types::*;
use resources::*;
use grid::*;
use controller_select::*;
use countdown::*;
use snake::*;
use fruits::*;
use input::*;
use game_tick::*;
use hud::*;
use round_over::*;

fn main() {
    App::new()
        // Window & default plugins configuration
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "2-Player Tron Snake (Bevy 2D)".into(),
                resolution: (1600, 900).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        // Background color
        .insert_resource(ClearColor(COLOR_BG))
        // Game resources
        .init_resource::<ArenaLayout>()
        .init_resource::<GameClock>()
        .init_resource::<CountdownState>()
        .init_resource::<ControllerAssignments>()
        .init_resource::<GameWinner>()
        .init_resource::<NotificationMessage>()
        // Game States
        .init_state::<GameState>()
        // Core persistent systems
        .add_systems(Startup, (setup_camera, setup_playfield))
        .add_systems(Update, (update_arena_layout, draw_arena_borders))
        // Controller Selection State
        .add_systems(OnEnter(GameState::ControllerSelect), setup_controller_select_ui)
        .add_systems(
            Update,
            handle_controller_select_input.run_if(in_state(GameState::ControllerSelect)),
        )
        .add_systems(OnExit(GameState::ControllerSelect), cleanup_controller_select_ui)
        // Countdown State
        .add_systems(
            OnEnter(GameState::Countdown),
            (spawn_initial_snakes, spawn_initial_fruits, setup_countdown_ui),
        )
        .add_systems(
            Update,
            (
                update_countdown,
                update_snake_head_visuals,
                animate_fruits,
            ).run_if(in_state(GameState::Countdown)),
        )
        .add_systems(OnExit(GameState::Countdown), cleanup_countdown_ui)
        // Active Playing State
        .add_systems(OnEnter(GameState::Playing), setup_hud)
        .add_systems(
            Update,
            (
                handle_gameplay_input,
                update_snake_head_visuals,
                game_clock_tick,
                animate_fruits,
                update_hud,
            ).run_if(in_state(GameState::Playing)),
        )
        // Round Over State
        .add_systems(OnEnter(GameState::RoundOver), setup_round_over_ui)
        .add_systems(
            Update,
            handle_round_over_input.run_if(in_state(GameState::RoundOver)),
        )
        .add_systems(
            OnExit(GameState::RoundOver),
            (cleanup_round_over_ui, cleanup_hud),
        )
        .run();
}
