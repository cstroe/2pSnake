use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;

pub fn handle_gameplay_input(
    mut commands: Commands,
    sound_assets: Res<SoundAssets>,
    gamepads: Query<(Entity, &Gamepad)>,
    keys: Res<ButtonInput<KeyCode>>,
    assignments: Res<ControllerAssignments>,
    mut game_clock: ResMut<GameClock>,
    mut notification: ResMut<NotificationMessage>,
    mut snakes: Query<&mut SnakeState>,
) {
    // 1. Tick speed adjustments
    let mut speed_delta = 0.0;

    if keys.just_pressed(KeyCode::Equal) || keys.just_pressed(KeyCode::BracketRight) {
        speed_delta -= TICK_STEP; // Faster ticks (less seconds per tick)
    }
    if keys.just_pressed(KeyCode::Minus) || keys.just_pressed(KeyCode::BracketLeft) {
        speed_delta += TICK_STEP; // Slower ticks (more seconds per tick)
    }

    // Also check gamepads for speed adjustments (LB = Faster, RB = Slower)
    for (_, gamepad) in &gamepads {
        if gamepad.just_pressed(GamepadButton::LeftTrigger) {
            speed_delta -= TICK_STEP;
        }
        if gamepad.just_pressed(GamepadButton::RightTrigger) {
            speed_delta += TICK_STEP;
        }
    }

    if speed_delta != 0.0 {
        game_clock.adjust_speed(speed_delta);
        notification.text = format!("Tick Speed: {:.2}s / tick", game_clock.tick_duration);
        notification.timer = Timer::from_seconds(2.0, TimerMode::Once);
    }

    // 2. Snake direction input for each snake
    for mut snake in &mut snakes {
        let assigned_device = match snake.player {
            PlayerId::Player1 => assignments.p1_device.as_ref(),
            PlayerId::Player2 => assignments.p2_device.as_ref(),
        };

        let mut desired_dir = None;

        if let Some(device) = assigned_device {
            match device {
                ControllerDevice::Gamepad(target_entity) => {
                    for (entity, gamepad) in &gamepads {
                        if entity == *target_entity {
                            // D-pad checks
                            if gamepad.just_pressed(GamepadButton::DPadUp) {
                                desired_dir = Some(Direction::Up);
                            } else if gamepad.just_pressed(GamepadButton::DPadDown) {
                                desired_dir = Some(Direction::Down);
                            } else if gamepad.just_pressed(GamepadButton::DPadLeft) {
                                desired_dir = Some(Direction::Left);
                            } else if gamepad.just_pressed(GamepadButton::DPadRight) {
                                desired_dir = Some(Direction::Right);
                            } else {
                                // Analog Stick checks
                                let sx = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
                                let sy = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);

                                if sx.abs() > 0.55 || sy.abs() > 0.55 {
                                    if sx.abs() > sy.abs() {
                                        if sx > 0.55 {
                                            desired_dir = Some(Direction::Right);
                                        } else if sx < -0.55 {
                                            desired_dir = Some(Direction::Left);
                                        }
                                    } else {
                                        if sy > 0.55 {
                                            desired_dir = Some(Direction::Up);
                                        } else if sy < -0.55 {
                                            desired_dir = Some(Direction::Down);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                ControllerDevice::KeyboardP1 => {
                    if keys.just_pressed(KeyCode::KeyW) {
                        desired_dir = Some(Direction::Up);
                    } else if keys.just_pressed(KeyCode::KeyS) {
                        desired_dir = Some(Direction::Down);
                    } else if keys.just_pressed(KeyCode::KeyA) {
                        desired_dir = Some(Direction::Left);
                    } else if keys.just_pressed(KeyCode::KeyD) {
                        desired_dir = Some(Direction::Right);
                    }
                }
                ControllerDevice::KeyboardP2 => {
                    if keys.just_pressed(KeyCode::ArrowUp) {
                        desired_dir = Some(Direction::Up);
                    } else if keys.just_pressed(KeyCode::ArrowDown) {
                        desired_dir = Some(Direction::Down);
                    } else if keys.just_pressed(KeyCode::ArrowLeft) {
                        desired_dir = Some(Direction::Left);
                    } else if keys.just_pressed(KeyCode::ArrowRight) {
                        desired_dir = Some(Direction::Right);
                    }
                }
            }
        }

        // Apply direction change if not reversing onto itself
        if let Some(new_dir) = desired_dir {
            if new_dir != snake.direction.opposite() {
                snake.next_direction = new_dir;
            }
        }

        // 3. Check for A Button / Speed Boost activation
        let mut boost_requested = false;
        if let Some(device) = assigned_device {
            match device {
                ControllerDevice::Gamepad(target_entity) => {
                    for (entity, gamepad) in &gamepads {
                        if entity == *target_entity {
                            if gamepad.just_pressed(GamepadButton::South) {
                                boost_requested = true;
                            }
                        }
                    }
                }
                ControllerDevice::KeyboardP1 => {
                    // Space, Left Shift, or KeyE for Player 1
                    if keys.just_pressed(KeyCode::Space)
                        || keys.just_pressed(KeyCode::ShiftLeft)
                        || keys.just_pressed(KeyCode::KeyE)
                    {
                        boost_requested = true;
                    }
                }
                ControllerDevice::KeyboardP2 => {
                    // Enter, Right Shift, Numpad 0, or Slash for Player 2
                    if keys.just_pressed(KeyCode::Enter)
                        || keys.just_pressed(KeyCode::ShiftRight)
                        || keys.just_pressed(KeyCode::Numpad0)
                        || keys.just_pressed(KeyCode::Slash)
                    {
                        boost_requested = true;
                    }
                }
            }
        }

        if boost_requested {
            if snake.stored_peppers > 0 {
                snake.stored_peppers -= 1;
                snake.boost_timer += 10.0;
                let player_name = match snake.player {
                    PlayerId::Player1 => "Player 1",
                    PlayerId::Player2 => "Player 2",
                };
                notification.text = format!(
                    "{} activated SPEED BOOST! (Active: {:.1}s)",
                    player_name, snake.boost_timer
                );
                notification.timer = Timer::from_seconds(2.0, TimerMode::Once);

                // Play activation sound effect
                commands.spawn((
                    AudioPlayer::new(sound_assets.powerup.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        }
    }
}
