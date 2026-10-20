use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;

pub fn setup_controller_select_ui(mut commands: Commands) {
    commands.spawn((
        ControllerSelectRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(30.0)),
            ..default()
        },
        BackgroundColor(COLOR_BG),
    )).with_children(|parent| {
        // --- HEADER ---
        parent.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        }).with_children(|header| {
            header.spawn((
                Text::new("2-PLAYER TRON SNAKE"),
                TextFont {
                    font_size: FontSize::Px(42.0),
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.9, 1.0)),
            ));

            header.spawn((
                Text::new("CONTROLLER SELECTION & READY ROOM"),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(COLOR_TEXT_MUTED),
            ));
        });

        // --- 3 COLUMN SLOTS ---
        parent.spawn(Node {
            width: Val::Percent(95.0),
            height: Val::Percent(65.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Stretch,
            column_gap: Val::Px(24.0),
            ..default()
        }).with_children(|columns| {
            // Player 1 Card (Left)
            columns.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(24.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(COLOR_PANEL_BG),
                BorderColor::all(COLOR_P1),
            )).with_children(|card| {
                card.spawn((
                    Text::new("PLAYER 1"),
                    TextFont {
                        font_size: FontSize::Px(32.0),
                        ..default()
                    },
                    TextColor(COLOR_P1),
                ));

                // Color swatch
                card.spawn((
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(40.0),
                        margin: UiRect::vertical(Val::Px(12.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(COLOR_P1),
                    BorderColor::all(Color::WHITE),
                ));

                card.spawn((
                    P1DeviceText,
                    Text::new("Assigned: None"),
                    TextFont {
                        font_size: FontSize::Px(18.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                card.spawn((
                    P1SlotStatusText,
                    Text::new("WAITING FOR PLAYER..."),
                    TextFont {
                        font_size: FontSize::Px(22.0),
                        ..default()
                    },
                    TextColor(COLOR_TEXT_MUTED),
                ));

                card.spawn((
                    Text::new("Press 'A' on Gamepad or 'Space' to Ready"),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(COLOR_TEXT_MUTED),
                ));
            });

            // Middle Column: Instructions & Devices list
            columns.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.1,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(20.0)),
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(COLOR_PANEL_BG),
                BorderColor::all(COLOR_ARENA_BORDER),
            )).with_children(|center| {
                center.spawn((
                    Text::new("DETECTED CONTROLLERS"),
                    TextFont {
                        font_size: FontSize::Px(24.0),
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 1.0)),
                ));

                center.spawn((
                    AvailableDevicesText,
                    Text::new("Scanning controllers..."),
                    TextFont {
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    TextColor(COLOR_TEXT_MUTED),
                ));

                center.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    row_gap: Val::Px(6.0),
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                }).with_children(|guide| {
                    guide.spawn((
                        Text::new("• D-Pad Left / Stick Left: Pick Player 1"),
                        TextFont { font_size: FontSize::Px(14.0), ..default() },
                        TextColor(COLOR_P1),
                    ));
                    guide.spawn((
                        Text::new("• D-Pad Right / Stick Right: Pick Player 2"),
                        TextFont { font_size: FontSize::Px(14.0), ..default() },
                        TextColor(COLOR_P2),
                    ));
                    guide.spawn((
                        Text::new("• Press 'A' Button: Ready Up / Confirm"),
                        TextFont { font_size: FontSize::Px(14.0), ..default() },
                        TextColor(COLOR_READY_GREEN),
                    ));
                    guide.spawn((
                        Text::new("• Keyboard: WASD + Space (P1) | Arrows + Enter (P2)"),
                        TextFont { font_size: FontSize::Px(14.0), ..default() },
                        TextColor(Color::WHITE),
                    ));
                });
            });

            // Player 2 Card (Right)
            columns.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(24.0)),
                    border: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(COLOR_PANEL_BG),
                BorderColor::all(COLOR_P2),
            )).with_children(|card| {
                card.spawn((
                    Text::new("PLAYER 2"),
                    TextFont {
                        font_size: FontSize::Px(32.0),
                        ..default()
                    },
                    TextColor(COLOR_P2),
                ));

                // Color swatch
                card.spawn((
                    Node {
                        width: Val::Px(120.0),
                        height: Val::Px(40.0),
                        margin: UiRect::vertical(Val::Px(12.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(COLOR_P2),
                    BorderColor::all(Color::WHITE),
                ));

                card.spawn((
                    P2DeviceText,
                    Text::new("Assigned: None"),
                    TextFont {
                        font_size: FontSize::Px(18.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                card.spawn((
                    P2SlotStatusText,
                    Text::new("WAITING FOR PLAYER..."),
                    TextFont {
                        font_size: FontSize::Px(22.0),
                        ..default()
                    },
                    TextColor(COLOR_TEXT_MUTED),
                ));

                card.spawn((
                    Text::new("Press 'A' on Gamepad or 'Enter' to Ready"),
                    TextFont {
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(COLOR_TEXT_MUTED),
                ));
            });
        });

        // --- BOTTOM STATUS BAR ---
        parent.spawn(Node {
            margin: UiRect::top(Val::Px(16.0)),
            ..default()
        }).with_children(|bottom| {
            bottom.spawn((
                BottomStatusText,
                Text::new("Both players must select a slot and press 'A' to start the match."),
                TextFont {
                    font_size: FontSize::Px(18.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    });
}

pub fn handle_controller_select_input(
    gamepads: Query<(Entity, &Gamepad, Option<&Name>)>,
    keys: Res<ButtonInput<KeyCode>>,
    mut assignments: ResMut<ControllerAssignments>,
    mut next_state: ResMut<NextState<GameState>>,
    mut p1_device_text: Query<&mut Text, (With<P1DeviceText>, Without<P2DeviceText>, Without<P1SlotStatusText>, Without<P2SlotStatusText>, Without<AvailableDevicesText>, Without<BottomStatusText>)>,
    mut p2_device_text: Query<&mut Text, (With<P2DeviceText>, Without<P1DeviceText>, Without<P1SlotStatusText>, Without<P2SlotStatusText>, Without<AvailableDevicesText>, Without<BottomStatusText>)>,
    mut p1_status_text: Query<(&mut Text, &mut TextColor), (With<P1SlotStatusText>, Without<P1DeviceText>, Without<P2DeviceText>, Without<P2SlotStatusText>, Without<AvailableDevicesText>, Without<BottomStatusText>)>,
    mut p2_status_text: Query<(&mut Text, &mut TextColor), (With<P2SlotStatusText>, Without<P1DeviceText>, Without<P2DeviceText>, Without<P1SlotStatusText>, Without<AvailableDevicesText>, Without<BottomStatusText>)>,
    mut devices_text: Query<&mut Text, (With<AvailableDevicesText>, Without<P1DeviceText>, Without<P2DeviceText>, Without<P1SlotStatusText>, Without<P2SlotStatusText>, Without<BottomStatusText>)>,
    mut bottom_status: Query<&mut Text, (With<BottomStatusText>, Without<P1DeviceText>, Without<P2DeviceText>, Without<P1SlotStatusText>, Without<P2SlotStatusText>, Without<AvailableDevicesText>)>,
) {
    let mut detected_gamepad_list = Vec::new();

    // 1. Check all connected gamepads
    for (entity, gamepad, name) in &gamepads {
        let name_str = name.map(|n| n.as_str()).unwrap_or("Gamepad");
        let display = format!("• {} [ID: {}]", name_str, entity.index());
        detected_gamepad_list.push(display);

        let stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);

        let is_left = gamepad.just_pressed(GamepadButton::DPadLeft) || stick_x < -0.5;
        let is_right = gamepad.just_pressed(GamepadButton::DPadRight) || stick_x > 0.5;
        let is_a = gamepad.just_pressed(GamepadButton::South);
        let is_b = gamepad.just_pressed(GamepadButton::East);

        let this_dev = ControllerDevice::Gamepad(entity);

        // Move to P1
        if is_left {
            if assignments.p1_device.as_ref() != Some(&this_dev) {
                if !assignments.p1_ready {
                    if assignments.p2_device.as_ref() == Some(&this_dev) {
                        assignments.p2_device = None;
                        assignments.p2_ready = false;
                    }
                    assignments.p1_device = Some(this_dev.clone());
                    assignments.p1_ready = false;
                }
            }
        }

        // Move to P2
        if is_right {
            if assignments.p2_device.as_ref() != Some(&this_dev) {
                if !assignments.p2_ready {
                    if assignments.p1_device.as_ref() == Some(&this_dev) {
                        assignments.p1_device = None;
                        assignments.p1_ready = false;
                    }
                    assignments.p2_device = Some(this_dev.clone());
                    assignments.p2_ready = false;
                }
            }
        }

        // Press A to Ready
        if is_a {
            if assignments.p1_device.as_ref() == Some(&this_dev) {
                assignments.p1_ready = !assignments.p1_ready;
            } else if assignments.p2_device.as_ref() == Some(&this_dev) {
                assignments.p2_ready = !assignments.p2_ready;
            } else {
                // If not assigned yet, assign to first free slot and ready!
                if assignments.p1_device.is_none() {
                    assignments.p1_device = Some(this_dev.clone());
                    assignments.p1_ready = true;
                } else if assignments.p2_device.is_none() {
                    assignments.p2_device = Some(this_dev.clone());
                    assignments.p2_ready = true;
                }
            }
        }

        // Press B to Unready
        if is_b {
            if assignments.p1_device.as_ref() == Some(&this_dev) {
                assignments.p1_ready = false;
            } else if assignments.p2_device.as_ref() == Some(&this_dev) {
                assignments.p2_ready = false;
            }
        }
    }

    // 2. Keyboard Support
    // Player 1 Keyboard: WASD + Space
    if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::KeyW) {
        if !assignments.p1_ready {
            assignments.p1_device = Some(ControllerDevice::KeyboardP1);
        }
    }
    if keys.just_pressed(KeyCode::Space) {
        if assignments.p1_device.as_ref() == Some(&ControllerDevice::KeyboardP1) {
            assignments.p1_ready = !assignments.p1_ready;
        } else if assignments.p1_device.is_none() {
            assignments.p1_device = Some(ControllerDevice::KeyboardP1);
            assignments.p1_ready = true;
        }
    }

    // Player 2 Keyboard: Arrows + Enter
    if keys.just_pressed(KeyCode::ArrowRight) || keys.just_pressed(KeyCode::ArrowUp) || keys.just_pressed(KeyCode::ArrowLeft) {
        if !assignments.p2_ready {
            assignments.p2_device = Some(ControllerDevice::KeyboardP2);
        }
    }
    if keys.just_pressed(KeyCode::Enter) {
        if assignments.p2_device.as_ref() == Some(&ControllerDevice::KeyboardP2) {
            assignments.p2_ready = !assignments.p2_ready;
        } else if assignments.p2_device.is_none() {
            assignments.p2_device = Some(ControllerDevice::KeyboardP2);
            assignments.p2_ready = true;
        }
    }

    // 3. Update UI texts
    if let Some(mut text) = p1_device_text.iter_mut().next() {
        **text = match &assignments.p1_device {
            Some(dev) => format!("Device: {}", dev.display_name(None)),
            None => "Device: [None Selected]".to_string(),
        };
    }

    if let Some(mut text) = p2_device_text.iter_mut().next() {
        **text = match &assignments.p2_device {
            Some(dev) => format!("Device: {}", dev.display_name(None)),
            None => "Device: [None Selected]".to_string(),
        };
    }

    if let Some((mut text, mut color)) = p1_status_text.iter_mut().next() {
        if assignments.p1_ready {
            **text = ">>> READY! <<<".to_string();
            color.0 = COLOR_READY_GREEN;
        } else if assignments.p1_device.is_some() {
            **text = "PRESS 'A' / SPACE TO READY".to_string();
            color.0 = Color::srgb(1.0, 0.8, 0.2);
        } else {
            **text = "WAITING FOR SELECTION...".to_string();
            color.0 = COLOR_TEXT_MUTED;
        }
    }

    if let Some((mut text, mut color)) = p2_status_text.iter_mut().next() {
        if assignments.p2_ready {
            **text = ">>> READY! <<<".to_string();
            color.0 = COLOR_READY_GREEN;
        } else if assignments.p2_device.is_some() {
            **text = "PRESS 'A' / ENTER TO READY".to_string();
            color.0 = Color::srgb(1.0, 0.8, 0.2);
        } else {
            **text = "WAITING FOR SELECTION...".to_string();
            color.0 = COLOR_TEXT_MUTED;
        }
    }

    if let Some(mut text) = devices_text.iter_mut().next() {
        let mut list_str = String::new();
        if detected_gamepad_list.is_empty() {
            list_str.push_str("• No USB Gamepads detected.\n  Plug in Xbox/USB controllers anytime!\n\n");
        } else {
            for pad in &detected_gamepad_list {
                list_str.push_str(&format!("{pad}\n"));
            }
            list_str.push('\n');
        }
        list_str.push_str("• Keyboard 1: WASD + Space\n• Keyboard 2: Arrows + Enter");
        **text = list_str;
    }

    // 4. Check if both players are ready
    if assignments.is_ready_to_start() {
        if let Some(mut text) = bottom_status.iter_mut().next() {
            **text = "BOTH PLAYERS READY! STARTING GAME...".to_string();
        }
        next_state.set(GameState::Countdown);
    }

    // 5. Back to Start Screen (Escape or B button)
    let mut go_back = keys.just_pressed(KeyCode::Escape);
    for (_, gamepad, _) in &gamepads {
        if gamepad.just_pressed(GamepadButton::East) {
            go_back = true;
            break;
        }
    }
    if go_back {
        assignments.reset_ready();
        next_state.set(GameState::StartScreen);
    }
}

pub fn cleanup_controller_select_ui(
    mut commands: Commands,
    query: Query<Entity, With<ControllerSelectRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
