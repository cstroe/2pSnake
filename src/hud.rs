use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;

pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        HudRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(HUD_HEIGHT_PX),
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::new(Val::Px(24.0), Val::Px(24.0), Val::Px(8.0), Val::Px(8.0)),
            border: UiRect::bottom(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(COLOR_HUD_BG),
        BorderColor::all(COLOR_ARENA_BORDER),
    )).with_children(|hud| {
        // --- LEFT: PLAYER 1 STATS ---
        hud.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            min_width: Val::Px(280.0),
            ..default()
        }).with_children(|p1_box| {
            p1_box.spawn((
                Text::new("PLAYER 1 (CYAN)"),
                TextFont { font_size: FontSize::Px(16.0), ..default() },
                TextColor(COLOR_P1),
            ));

            p1_box.spawn((
                HudP1ScoreText,
                Text::new("Fruits: 0 / 15  |  Len: 6"),
                TextFont { font_size: FontSize::Px(20.0), ..default() },
                TextColor(Color::WHITE),
            ));
        });

        // --- CENTER: TIME, SPEED & MESSAGES ---
        hud.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            min_width: Val::Px(360.0),
            ..default()
        }).with_children(|center| {
            center.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                align_items: AlignItems::Center,
                ..default()
            }).with_children(|row| {
                row.spawn((
                    HudTimeText,
                    Text::new("TIME: 05:00"),
                    TextFont { font_size: FontSize::Px(22.0), ..default() },
                    TextColor(Color::srgb(1.0, 0.9, 0.3)),
                ));

                row.spawn((
                    HudSpeedText,
                    Text::new("TICK: 0.20s"),
                    TextFont { font_size: FontSize::Px(16.0), ..default() },
                    TextColor(COLOR_TEXT_MUTED),
                ));
            });

            center.spawn((
                HudMessageText,
                Text::new("Grid: 650 x 400 | First to 15 Fruits Wins!"),
                TextFont { font_size: FontSize::Px(13.0), ..default() },
                TextColor(COLOR_TEXT_MUTED),
            ));
        });

        // --- RIGHT: PLAYER 2 STATS ---
        hud.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexEnd,
            min_width: Val::Px(280.0),
            ..default()
        }).with_children(|p2_box| {
            p2_box.spawn((
                Text::new("PLAYER 2 (ORANGE)"),
                TextFont { font_size: FontSize::Px(16.0), ..default() },
                TextColor(COLOR_P2),
            ));

            p2_box.spawn((
                HudP2ScoreText,
                Text::new("Fruits: 0 / 15  |  Len: 6"),
                TextFont { font_size: FontSize::Px(20.0), ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn update_hud(
    game_clock: Res<GameClock>,
    notification: Res<NotificationMessage>,
    snakes: Query<&SnakeState>,
    mut p1_score_text: Query<&mut Text, (With<HudP1ScoreText>, Without<HudP2ScoreText>, Without<HudTimeText>, Without<HudSpeedText>, Without<HudMessageText>)>,
    mut p2_score_text: Query<&mut Text, (With<HudP2ScoreText>, Without<HudP1ScoreText>, Without<HudTimeText>, Without<HudSpeedText>, Without<HudMessageText>)>,
    mut time_text: Query<&mut Text, (With<HudTimeText>, Without<HudP1ScoreText>, Without<HudP2ScoreText>, Without<HudSpeedText>, Without<HudMessageText>)>,
    mut speed_text: Query<&mut Text, (With<HudSpeedText>, Without<HudP1ScoreText>, Without<HudP2ScoreText>, Without<HudTimeText>, Without<HudMessageText>)>,
    mut message_text: Query<&mut Text, (With<HudMessageText>, Without<HudP1ScoreText>, Without<HudP2ScoreText>, Without<HudTimeText>, Without<HudSpeedText>)>,
) {
    // 1. Update Player 1 & 2 Scores
    for snake in &snakes {
        if snake.player == PlayerId::Player1 {
            if let Some(mut text) = p1_score_text.iter_mut().next() {
                **text = format!("Fruits: {} / {}  |  Len: {}", snake.score, TARGET_FRUITS_TO_WIN, snake.body.len());
            }
        } else {
            if let Some(mut text) = p2_score_text.iter_mut().next() {
                **text = format!("Fruits: {} / {}  |  Len: {}", snake.score, TARGET_FRUITS_TO_WIN, snake.body.len());
            }
        }
    }

    // 2. Update Round Time
    if let Some(mut text) = time_text.iter_mut().next() {
        let total_secs = game_clock.round_time_remaining as u32;
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        **text = format!("TIME: {:02}:{:02}", mins, secs);
    }

    // 3. Update Tick Speed
    if let Some(mut text) = speed_text.iter_mut().next() {
        **text = format!("TICK: {:.2}s", game_clock.tick_duration);
    }

    // 4. Update Notification message
    if let Some(mut text) = message_text.iter_mut().next() {
        if !notification.timer.is_finished() {
            **text = notification.text.clone();
        } else {
            **text = "Controls: D-Pad / Stick / WASD / Arrows | Speed: LB/RB or +/-".to_string();
        }
    }
}

pub fn cleanup_hud(
    mut commands: Commands,
    query: Query<Entity, With<HudRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
