use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;
use crate::svg_utils::SpriteAssets;

const MENU_LABELS: [&str; 4] = [
    "(1) START GAME",
    "(2) SETTINGS",
    "(3) CREDITS",
    "(4) EXIT",
];

// ============================================================================
// 1. START SCREEN
// ============================================================================

pub fn setup_start_screen_ui(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    menu_state: Res<StartMenuState>,
) {
    commands.spawn((
        StartScreenRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(40.0)),
            ..default()
        },
        BackgroundColor(COLOR_BG),
    )).with_children(|parent| {
        // --- HEADER & TITLE ---
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::top(Val::Px(16.0)),
            ..default()
        }).with_children(|header| {
            header.spawn((
                Text::new("2-PLAYER TRON SNAKE"),
                TextFont {
                    font_size: FontSize::Px(52.0),
                    ..default()
                },
                TextColor(COLOR_P1),
            ));

            header.spawn((
                Text::new("A RETRO CYBERPUNK BATTLE"),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.3)),
            ));

            // Decorative showcase row
            header.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(20.0),
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            }).with_children(|icons| {
                // P1 Head
                icons.spawn((
                    Node {
                        width: Val::Px(44.0),
                        height: Val::Px(44.0),
                        border: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BorderColor::all(COLOR_P1),
                    BackgroundColor(Color::srgba(0.0, 0.9, 1.0, 0.15)),
                    ImageNode::new(sprites.p1_head.clone()),
                ));

                // Hot Pepper
                icons.spawn((
                    Node {
                        width: Val::Px(36.0),
                        height: Val::Px(36.0),
                        ..default()
                    },
                    ImageNode::new(sprites.fruit_pepper.clone()),
                ));

                // P2 Head
                icons.spawn((
                    Node {
                        width: Val::Px(44.0),
                        height: Val::Px(44.0),
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

        // --- MENU OPTIONS ---
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(16.0),
            margin: UiRect::vertical(Val::Px(20.0)),
            ..default()
        }).with_children(|menu| {
            for (index, &label) in MENU_LABELS.iter().enumerate() {
                let is_selected = index == menu_state.selected_index;
                let border_color = if is_selected { COLOR_P1 } else { COLOR_ARENA_BORDER };
                let bg_color = if is_selected {
                    Color::srgba(0.0, 0.9, 1.0, 0.20)
                } else {
                    COLOR_PANEL_BG
                };
                let display_text = if is_selected {
                    format!("▶  {label}")
                } else {
                    format!("   {label}")
                };
                let text_color = if is_selected { Color::WHITE } else { COLOR_TEXT_MUTED };

                menu.spawn((
                    StartMenuItemNode(index),
                    Node {
                        width: Val::Px(380.0),
                        height: Val::Px(52.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        padding: UiRect::horizontal(Val::Px(20.0)),
                        ..default()
                    },
                    BorderColor::all(border_color),
                    BackgroundColor(bg_color),
                )).with_children(|btn| {
                    btn.spawn((
                        StartMenuItemText(index),
                        Text::new(display_text),
                        TextFont {
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                        TextColor(text_color),
                    ));
                });
            }
        });

        // --- CONTROLS FOOTER ---
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(6.0),
            margin: UiRect::bottom(Val::Px(12.0)),
            ..default()
        }).with_children(|footer| {
            footer.spawn((
                Text::new("NAVIGATE: UP / DOWN  |  W / S  |  D-PAD"),
                TextFont {
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(COLOR_READY_GREEN),
            ));

            footer.spawn((
                Text::new("SELECT: 'A' / ENTER / SPACE / KEYS 1 - 4"),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextColor(COLOR_TEXT_MUTED),
            ));
        });
    });
}

pub fn handle_start_screen_input(
    mut commands: Commands,
    sound_assets: Res<SoundAssets>,
    gamepads: Query<&Gamepad>,
    keys: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<StartMenuState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit_writer: MessageWriter<AppExit>,
    mut item_nodes: Query<(&mut BorderColor, &mut BackgroundColor, &StartMenuItemNode)>,
    mut item_texts: Query<(&mut Text, &mut TextColor, &StartMenuItemText)>,
) {
    let mut move_up = keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::KeyW);
    let mut move_down = keys.just_pressed(KeyCode::ArrowDown) || keys.just_pressed(KeyCode::KeyS);
    let mut trigger_select = keys.just_pressed(KeyCode::Enter) || keys.just_pressed(KeyCode::Space);

    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::DPadUp) {
            move_up = true;
        }
        if gamepad.just_pressed(GamepadButton::DPadDown) {
            move_down = true;
        }
        if gamepad.just_pressed(GamepadButton::South) || gamepad.just_pressed(GamepadButton::Start) {
            trigger_select = true;
        }
    }

    // Direct number shortcuts: (1) Start Game, (2) Settings, (3) Credits, (4) Exit
    if keys.just_pressed(KeyCode::Digit1) || keys.just_pressed(KeyCode::Numpad1) {
        menu_state.selected_index = 0;
        trigger_select = true;
    } else if keys.just_pressed(KeyCode::Digit2) || keys.just_pressed(KeyCode::Numpad2) {
        menu_state.selected_index = 1;
        trigger_select = true;
    } else if keys.just_pressed(KeyCode::Digit3) || keys.just_pressed(KeyCode::Numpad3) {
        menu_state.selected_index = 2;
        trigger_select = true;
    } else if keys.just_pressed(KeyCode::Digit4) || keys.just_pressed(KeyCode::Numpad4) {
        menu_state.selected_index = 3;
        trigger_select = true;
    }

    // Move selection
    if move_up {
        menu_state.selected_index = (menu_state.selected_index + 3) % 4;
    }
    if move_down {
        menu_state.selected_index = (menu_state.selected_index + 1) % 4;
    }

    // Update visuals
    for (mut border, mut bg, node) in &mut item_nodes {
        let is_sel = node.0 == menu_state.selected_index;
        *border = BorderColor::all(if is_sel { COLOR_P1 } else { COLOR_ARENA_BORDER });
        *bg = BackgroundColor(if is_sel {
            Color::srgba(0.0, 0.9, 1.0, 0.20)
        } else {
            COLOR_PANEL_BG
        });
    }

    for (mut text, mut color, item_text) in &mut item_texts {
        let is_sel = item_text.0 == menu_state.selected_index;
        let label = MENU_LABELS[item_text.0];
        **text = if is_sel {
            format!("▶  {label}")
        } else {
            format!("   {label}")
        };
        *color = TextColor(if is_sel { Color::WHITE } else { COLOR_TEXT_MUTED });
    }

    // Execute selected action
    if trigger_select {
        commands.spawn((
            AudioPlayer::new(sound_assets.eat_fruit.clone()),
            PlaybackSettings::DESPAWN,
        ));

        match menu_state.selected_index {
            0 => next_state.set(GameState::ControllerSelect),
            1 => next_state.set(GameState::Settings),
            2 => next_state.set(GameState::Credits),
            3 => {
                exit_writer.write(AppExit::Success);
            }
            _ => {}
        }
    }
}

pub fn cleanup_start_screen_ui(
    mut commands: Commands,
    query: Query<Entity, With<StartScreenRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ============================================================================
// 2. SETTINGS SKELETON SCREEN
// ============================================================================

pub fn setup_settings_ui(mut commands: Commands) {
    commands.spawn((
        SettingsScreenRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(40.0)),
            ..default()
        },
        BackgroundColor(COLOR_BG),
    )).with_children(|parent| {
        // --- HEADER ---
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::top(Val::Px(16.0)),
            ..default()
        }).with_children(|header| {
            header.spawn((
                Text::new("SETTINGS"),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(COLOR_P1),
            ));

            header.spawn((
                Text::new("GAME CONFIGURATION (SKELETON SCREEN)"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.3)),
            ));
        });

        // --- CONFIGURATION CARD ---
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Px(720.0),
                padding: UiRect::all(Val::Px(28.0)),
                row_gap: Val::Px(16.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(COLOR_PANEL_BG),
            BorderColor::all(COLOR_ARENA_BORDER),
        )).with_children(|card| {
            let settings_rows = [
                ("• Audio System", "Enabled (fastsong.mp3 BGM + SFX Powerups)"),
                ("• Match Duration", "5:00 Round Timer (Unlimited fruit collection)"),
                ("• Arena Dimensions", "112 x 70 Grid Cells (Auto-scaled 16:9)"),
                ("• Game Tick Speed", "0.20s default (In-game adjust: LB/RB or +/-)"),
                ("• Speed Boost", "Hot Pepper (Stored up to 3, press 'A' for 10s boost)"),
                ("• Controls - Player 1", "Gamepad (D-Pad/Stick) or Keyboard WASD + Space"),
                ("• Controls - Player 2", "Gamepad (D-Pad/Stick) or Keyboard Arrows + Enter"),
                ("• Display Mode", "Windowed / Maximized / Auto-scaling resizable"),
            ];

            for (title, value) in settings_rows {
                card.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                }).with_children(|row| {
                    row.spawn((
                        Text::new(title),
                        TextFont { font_size: FontSize::Px(16.0), ..default() },
                        TextColor(COLOR_P1),
                    ));

                    row.spawn((
                        Text::new(value),
                        TextFont { font_size: FontSize::Px(15.0), ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
            }
        });

        // --- FOOTER / RETURN PROMPT ---
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(16.0)),
            ..default()
        }).with_children(|footer| {
            footer.spawn((
                Text::new("PRESS 'B' ON GAMEPAD OR ESC / BACKSPACE / SPACE TO RETURN TO MAIN MENU"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(COLOR_READY_GREEN),
            ));
        });
    });
}

pub fn handle_settings_input(
    gamepads: Query<&Gamepad>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut go_back = keys.just_pressed(KeyCode::Escape)
        || keys.just_pressed(KeyCode::Backspace)
        || keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::Enter);

    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::East)
            || gamepad.just_pressed(GamepadButton::South)
            || gamepad.just_pressed(GamepadButton::Start)
        {
            go_back = true;
            break;
        }
    }

    if go_back {
        next_state.set(GameState::StartScreen);
    }
}

pub fn cleanup_settings_ui(
    mut commands: Commands,
    query: Query<Entity, With<SettingsScreenRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// ============================================================================
// 3. CREDITS SKELETON SCREEN
// ============================================================================

pub fn setup_credits_ui(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
) {
    commands.spawn((
        CreditsScreenRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(40.0)),
            ..default()
        },
        BackgroundColor(COLOR_BG),
    )).with_children(|parent| {
        // --- HEADER ---
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::top(Val::Px(16.0)),
            ..default()
        }).with_children(|header| {
            header.spawn((
                Text::new("CREDITS"),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(COLOR_P2),
            ));

            header.spawn((
                Text::new("2-PLAYER TRON SNAKE DEVELOPMENT TEAM"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.85, 0.3)),
            ));
        });

        // --- CREDITS CARD ---
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Px(720.0),
                padding: UiRect::all(Val::Px(28.0)),
                row_gap: Val::Px(16.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(COLOR_PANEL_BG),
            BorderColor::all(COLOR_ARENA_BORDER),
        )).with_children(|card| {
            let credits_rows = [
                ("• Game Framework", "Bevy Engine 0.20 (Rust 2D ECS Framework)"),
                ("• Vector Graphics Engine", "Resvg & Tiny-Skia Antialiased Dynamic Rasterizer"),
                ("• Audio Decoders", "Symphonia (MP3) & Hound (WAV) via Rodio"),
                ("• Background Soundtrack", "fastsong.mp3 (Looped Active Round Playback)"),
                ("• Sound Effects", "SFX_Powerup_01.wav (Fruit Eating & Speed Boost)"),
                ("• Art & UI Aesthetic", "Retro Cyberpunk Neon Glow & Vector Sprites"),
                ("• Game Rules & Design", "Tron Snake Duel: Box-in Tactics & Pepper Boosts"),
            ];

            for (title, value) in credits_rows {
                card.spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                }).with_children(|row| {
                    row.spawn((
                        Text::new(title),
                        TextFont { font_size: FontSize::Px(16.0), ..default() },
                        TextColor(COLOR_P2),
                    ));

                    row.spawn((
                        Text::new(value),
                        TextFont { font_size: FontSize::Px(15.0), ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
            }

            // Decorative row inside card
            card.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(24.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::top(Val::Px(12.0)),
                ..default()
            }).with_children(|row| {
                row.spawn((
                    Node {
                        width: Val::Px(36.0),
                        height: Val::Px(36.0),
                        ..default()
                    },
                    ImageNode::new(sprites.p1_head.clone()),
                ));
                row.spawn((
                    Node {
                        width: Val::Px(30.0),
                        height: Val::Px(30.0),
                        ..default()
                    },
                    ImageNode::new(sprites.fruit_pepper.clone()),
                ));
                row.spawn((
                    Node {
                        width: Val::Px(36.0),
                        height: Val::Px(36.0),
                        ..default()
                    },
                    ImageNode::new(sprites.p2_head.clone()),
                ));
            });
        });

        // --- FOOTER / RETURN PROMPT ---
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(16.0)),
            ..default()
        }).with_children(|footer| {
            footer.spawn((
                Text::new("PRESS 'B' ON GAMEPAD OR ESC / BACKSPACE / SPACE TO RETURN TO MAIN MENU"),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(COLOR_READY_GREEN),
            ));
        });
    });
}

pub fn handle_credits_input(
    gamepads: Query<&Gamepad>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut go_back = keys.just_pressed(KeyCode::Escape)
        || keys.just_pressed(KeyCode::Backspace)
        || keys.just_pressed(KeyCode::Space)
        || keys.just_pressed(KeyCode::Enter);

    for gamepad in &gamepads {
        if gamepad.just_pressed(GamepadButton::East)
            || gamepad.just_pressed(GamepadButton::South)
            || gamepad.just_pressed(GamepadButton::Start)
        {
            go_back = true;
            break;
        }
    }

    if go_back {
        next_state.set(GameState::StartScreen);
    }
}

pub fn cleanup_credits_ui(
    mut commands: Commands,
    query: Query<Entity, With<CreditsScreenRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
