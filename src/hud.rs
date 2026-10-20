use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;
use crate::svg_utils::SpriteAssets;

pub fn setup_hud(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
) {
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
        // --- LEFT: PLAYER 1 STATS & STORED PEPPERS ---
        hud.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            min_width: Val::Px(340.0),
            ..default()
        }).with_children(|p1_box| {
            // Player 1 Avatar Icon
            p1_box.spawn((
                Node {
                    width: Val::Px(42.0),
                    height: Val::Px(42.0),
                    border: UiRect::all(Val::Px(2.0)),
                    padding: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor::all(COLOR_P1),
                BackgroundColor(Color::srgba(0.0, 0.9, 1.0, 0.15)),
                ImageNode::new(sprites.p1_head.clone()),
            ));

            // Info column
            p1_box.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                row_gap: Val::Px(2.0),
                ..default()
            }).with_children(|info| {
                // Name & Score row
                info.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                }).with_children(|row| {
                    row.spawn((
                        Text::new("PLAYER 1"),
                        TextFont { font_size: FontSize::Px(16.0), ..default() },
                        TextColor(COLOR_P1),
                    ));

                    row.spawn((
                        HudP1ScoreText,
                        Text::new("Fruits: 0 | Len: 6"),
                        TextFont { font_size: FontSize::Px(16.0), ..default() },
                        TextColor(Color::WHITE),
                    ));
                });

                // Stored Peppers & Boost Row
                info.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(6.0),
                    ..default()
                }).with_children(|pep_row| {
                    for slot_index in 0..3 {
                        pep_row.spawn((
                            HudPlayerPepperSlot { player: PlayerId::Player1, slot_index },
                            Node {
                                width: Val::Px(22.0),
                                height: Val::Px(22.0),
                                ..default()
                            },
                            ImageNode {
                                image: sprites.fruit_pepper.clone(),
                                color: Color::srgba(1.0, 1.0, 1.0, 0.18),
                                ..default()
                            },
                        ));
                    }

                    pep_row.spawn((
                        HudPlayerBoostText { player: PlayerId::Player1 },
                        Text::new("Peppers: 0/3"),
                        TextFont { font_size: FontSize::Px(13.0), ..default() },
                        TextColor(COLOR_TEXT_MUTED),
                    ));
                });
            });
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
                Text::new("Grid: 112 x 70 | Collect unlimited fruits until time expires!"),
                TextFont { font_size: FontSize::Px(13.0), ..default() },
                TextColor(COLOR_TEXT_MUTED),
            ));
        });

        // --- RIGHT: PLAYER 2 STATS & STORED PEPPERS ---
        hud.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(12.0),
            min_width: Val::Px(340.0),
            justify_content: JustifyContent::FlexEnd,
            ..default()
        }).with_children(|p2_box| {
            // Info column
            p2_box.spawn(Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                row_gap: Val::Px(2.0),
                ..default()
            }).with_children(|info| {
                // Name & Score row
                info.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(10.0),
                    ..default()
                }).with_children(|row| {
                    row.spawn((
                        HudP2ScoreText,
                        Text::new("Fruits: 0 | Len: 6"),
                        TextFont { font_size: FontSize::Px(16.0), ..default() },
                        TextColor(Color::WHITE),
                    ));

                    row.spawn((
                        Text::new("PLAYER 2"),
                        TextFont { font_size: FontSize::Px(16.0), ..default() },
                        TextColor(COLOR_P2),
                    ));
                });

                // Stored Peppers & Boost Row
                info.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(6.0),
                    ..default()
                }).with_children(|pep_row| {
                    pep_row.spawn((
                        HudPlayerBoostText { player: PlayerId::Player2 },
                        Text::new("Peppers: 0/3"),
                        TextFont { font_size: FontSize::Px(13.0), ..default() },
                        TextColor(COLOR_TEXT_MUTED),
                    ));

                    for slot_index in 0..3 {
                        pep_row.spawn((
                            HudPlayerPepperSlot { player: PlayerId::Player2, slot_index },
                            Node {
                                width: Val::Px(22.0),
                                height: Val::Px(22.0),
                                ..default()
                            },
                            ImageNode {
                                image: sprites.fruit_pepper.clone(),
                                color: Color::srgba(1.0, 1.0, 1.0, 0.18),
                                ..default()
                            },
                        ));
                    }
                });
            });

            // Player 2 Avatar Icon
            p2_box.spawn((
                Node {
                    width: Val::Px(42.0),
                    height: Val::Px(42.0),
                    border: UiRect::all(Val::Px(2.0)),
                    padding: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BorderColor::all(COLOR_P2),
                BackgroundColor(Color::srgba(1.0, 0.4, 0.05, 0.15)),
                ImageNode::new(sprites.p2_head.clone()),
            ));
        });
    });
}

pub fn update_hud(
    game_clock: Res<GameClock>,
    notification: Res<NotificationMessage>,
    snakes: Query<&SnakeState>,
    mut p1_score_text: Query<&mut Text, (With<HudP1ScoreText>, Without<HudP2ScoreText>, Without<HudTimeText>, Without<HudSpeedText>, Without<HudMessageText>, Without<HudPlayerBoostText>)>,
    mut p2_score_text: Query<&mut Text, (With<HudP2ScoreText>, Without<HudP1ScoreText>, Without<HudTimeText>, Without<HudSpeedText>, Without<HudMessageText>, Without<HudPlayerBoostText>)>,
    mut time_text: Query<&mut Text, (With<HudTimeText>, Without<HudP1ScoreText>, Without<HudP2ScoreText>, Without<HudSpeedText>, Without<HudMessageText>, Without<HudPlayerBoostText>)>,
    mut speed_text: Query<&mut Text, (With<HudSpeedText>, Without<HudP1ScoreText>, Without<HudP2ScoreText>, Without<HudTimeText>, Without<HudMessageText>, Without<HudPlayerBoostText>)>,
    mut message_text: Query<&mut Text, (With<HudMessageText>, Without<HudP1ScoreText>, Without<HudP2ScoreText>, Without<HudTimeText>, Without<HudSpeedText>, Without<HudPlayerBoostText>)>,
    mut pepper_slots: Query<(&mut ImageNode, &HudPlayerPepperSlot)>,
    mut boost_texts: Query<(&mut Text, &mut TextColor, &HudPlayerBoostText), (Without<HudP1ScoreText>, Without<HudP2ScoreText>, Without<HudTimeText>, Without<HudSpeedText>, Without<HudMessageText>)>,
) {
    let mut p1_peppers = 0;
    let mut p2_peppers = 0;
    let mut p1_boost = 0.0;
    let mut p2_boost = 0.0;

    // 1. Update Player 1 & 2 Scores
    for snake in &snakes {
        if snake.player == PlayerId::Player1 {
            p1_peppers = snake.stored_peppers;
            p1_boost = snake.boost_timer;
            if let Some(mut text) = p1_score_text.iter_mut().next() {
                **text = format!("Fruits: {} | Len: {}", snake.score, snake.body.len());
            }
        } else {
            p2_peppers = snake.stored_peppers;
            p2_boost = snake.boost_timer;
            if let Some(mut text) = p2_score_text.iter_mut().next() {
                **text = format!("Fruits: {} | Len: {}", snake.score, snake.body.len());
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
            **text = "Controls: D-Pad / WASD / Arrows | Speed: LB/RB or +/- | [A] Use Pepper Boost".to_string();
        }
    }

    // 5. Update Stored Peppers visual indicators
    for (mut image_node, slot) in &mut pepper_slots {
        let pepper_count = match slot.player {
            PlayerId::Player1 => p1_peppers,
            PlayerId::Player2 => p2_peppers,
        };
        if slot.slot_index < pepper_count as usize {
            image_node.color = Color::WHITE; // Fully lit
        } else {
            image_node.color = Color::srgba(1.0, 1.0, 1.0, 0.18); // Faint outline slot
        }
    }

    // 6. Update Boost status text
    for (mut text, mut text_color, boost_marker) in &mut boost_texts {
        let (boost_timer, stored) = match boost_marker.player {
            PlayerId::Player1 => (p1_boost, p1_peppers),
            PlayerId::Player2 => (p2_boost, p2_peppers),
        };

        if boost_timer > 0.0 {
            **text = format!("⚡ BOOST: {:.1}s", boost_timer);
            *text_color = TextColor(Color::srgb(1.0, 0.9, 0.2));
        } else if stored > 0 {
            **text = format!("Press [A] to BOOST ({}x)", stored);
            *text_color = TextColor(Color::srgb(1.0, 0.45, 0.1));
        } else {
            **text = "Peppers: 0/3".to_string();
            *text_color = TextColor(COLOR_TEXT_MUTED);
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
