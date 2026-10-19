use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;
use crate::snake::cleanup_snakes;
use crate::fruits::cleanup_fruits;

pub fn setup_round_over_ui(
    mut commands: Commands,
    winner: Res<GameWinner>,
) {
    let (winner_title, title_color) = match winner.winner {
        Some(PlayerId::Player1) => ("VICTORY: PLAYER 1 WINS!", COLOR_P1),
        Some(PlayerId::Player2) => ("VICTORY: PLAYER 2 WINS!", COLOR_P2),
        None => ("MATCH OVER: IT'S A DRAW!", Color::srgb(1.0, 0.85, 0.3)),
    };

    commands.spawn((
        RoundOverRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(24.0),
            padding: UiRect::all(Val::Px(40.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.03, 0.06, 0.92)),
    )).with_children(|parent| {
        // Winner Title
        parent.spawn((
            RoundOverWinnerText,
            Text::new(winner_title),
            TextFont {
                font_size: FontSize::Px(48.0),
                ..default()
            },
            TextColor(title_color),
        ));

        // Subtitle / Reason
        parent.spawn((
            RoundOverDetailsText,
            Text::new(&winner.reason),
            TextFont {
                font_size: FontSize::Px(22.0),
                ..default()
            },
            TextColor(COLOR_TEXT_MUTED),
        ));

        // Final score box
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(50.0),
                padding: UiRect::all(Val::Px(24.0)),
                border: UiRect::all(Val::Px(2.0)),
                margin: UiRect::vertical(Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(COLOR_PANEL_BG),
            BorderColor::all(COLOR_ARENA_BORDER),
        )).with_children(|box_row| {
            // P1
            box_row.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            }).with_children(|p1| {
                p1.spawn((
                    Text::new("PLAYER 1"),
                    TextFont { font_size: FontSize::Px(20.0), ..default() },
                    TextColor(COLOR_P1),
                ));
                p1.spawn((
                    Text::new(format!("Fruits: {}", winner.p1_final_score)),
                    TextFont { font_size: FontSize::Px(28.0), ..default() },
                    TextColor(Color::WHITE),
                ));
            });

            // VS divider
            box_row.spawn((
                Text::new("VS"),
                TextFont { font_size: FontSize::Px(24.0), ..default() },
                TextColor(COLOR_TEXT_MUTED),
            ));

            // P2
            box_row.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            }).with_children(|p2| {
                p2.spawn((
                    Text::new("PLAYER 2"),
                    TextFont { font_size: FontSize::Px(20.0), ..default() },
                    TextColor(COLOR_P2),
                ));
                p2.spawn((
                    Text::new(format!("Fruits: {}", winner.p2_final_score)),
                    TextFont { font_size: FontSize::Px(28.0), ..default() },
                    TextColor(Color::WHITE),
                ));
            });
        });

        // Prompt to play again
        parent.spawn((
            Text::new("PRESS 'A' ON GAMEPAD OR SPACE / ENTER TO REMATCH"),
            TextFont {
                font_size: FontSize::Px(22.0),
                ..default()
            },
            TextColor(COLOR_READY_GREEN),
        ));
    });
}

pub fn handle_round_over_input(
    mut commands: Commands,
    gamepads: Query<&Gamepad>,
    keys: Res<ButtonInput<KeyCode>>,
    mut game_clock: ResMut<GameClock>,
    mut assignments: ResMut<ControllerAssignments>,
    mut next_state: ResMut<NextState<GameState>>,
    snakes: Query<(Entity, &SnakeState)>,
    segments: Query<Entity, With<SnakeSegment>>,
    heads: Query<Entity, With<SnakeHead>>,
    fruits: Query<Entity, With<Fruit>>,
) {
    let mut restart_triggered = keys.just_pressed(KeyCode::Space) || keys.just_pressed(KeyCode::Enter);

    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::South) || gamepad.just_pressed(GamepadButton::Start) {
            restart_triggered = true;
            break;
        }
    }

    if restart_triggered {
        // Reset game state
        game_clock.reset();
        assignments.reset_ready();

        cleanup_snakes(commands.reborrow(), snakes, segments, heads);
        cleanup_fruits(commands, fruits);

        next_state.set(GameState::ControllerSelect);
    }
}

pub fn cleanup_round_over_ui(
    mut commands: Commands,
    query: Query<Entity, With<RoundOverRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
