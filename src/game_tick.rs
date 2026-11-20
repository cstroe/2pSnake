use bevy::prelude::*;
use bevy::audio::Volume;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;
use crate::snake::respawn_snake;
use crate::fruits::spawn_one_fruit;
use crate::svg_utils::SpriteAssets;

pub fn game_clock_tick(
    mut commands: Commands,
    time: Res<Time>,
    layout: Res<ArenaLayout>,
    sprites: Res<SpriteAssets>,
    sound_assets: Res<SoundAssets>,
    mut game_clock: ResMut<GameClock>,
    mut notification: ResMut<NotificationMessage>,
    mut winner_info: ResMut<GameWinner>,
    mut next_state: ResMut<NextState<GameState>>,
    mut snakes: Query<&mut SnakeState>,
    mut head_transforms: Query<&mut Transform, (With<SnakeHead>, Without<SnakeSegment>, Without<Fruit>)>,
    mut segment_transforms: Query<&mut Transform, (With<SnakeSegment>, Without<SnakeHead>, Without<Fruit>)>,
    fruits_query: Query<(Entity, &Fruit)>,
) {
    // 1. Tick notification timer
    notification.timer.tick(time.delta());

    // 2. Tick round time
    game_clock.round_time_remaining = (game_clock.round_time_remaining - time.delta_secs()).max(0.0);
    if game_clock.round_time_remaining <= 0.0 {
        let mut p1_score = 0;
        let mut p2_score = 0;
        for snake in &snakes {
            if snake.player == PlayerId::Player1 {
                p1_score = snake.score;
            } else {
                p2_score = snake.score;
            }
        }
        winner_info.p1_final_score = p1_score;
        winner_info.p2_final_score = p2_score;

        if p1_score > p2_score {
            winner_info.winner = Some(PlayerId::Player1);
            winner_info.reason = format!("Time expired! Player 1 collected the most fruits ({p1_score} vs {p2_score})!");
        } else if p2_score > p1_score {
            winner_info.winner = Some(PlayerId::Player2);
            winner_info.reason = format!("Time expired! Player 2 collected the most fruits ({p2_score} vs {p1_score})!");
        } else {
            winner_info.winner = None;
            winner_info.reason = format!("Time expired! The match ended in a Draw ({p1_score} each)!");
        }

        next_state.set(GameState::RoundOver);
        return;
    }

    // 3. Tick per-snake movement and boost timers
    let dt = time.delta_secs();

    // Separate Player 1 and Player 2 explicitly so both can be borrowed independently
    let mut p1_opt = None;
    let mut p2_opt = None;
    for snake in &mut snakes {
        if snake.player == PlayerId::Player1 {
            p1_opt = Some(snake);
        } else {
            p2_opt = Some(snake);
        }
    }
    let (Some(mut p1), Some(mut p2)) = (p1_opt, p2_opt) else {
        return;
    };

    // Decrement boost timers
    p1.boost_timer = (p1.boost_timer - dt).max(0.0);
    p2.boost_timer = (p2.boost_timer - dt).max(0.0);

    let mut p1_just_respawned = false;
    let mut p2_just_respawned = false;

    // Handle dying snakes and respawn after the 1-second transition finishes
    if p1.death_timer > 0.0 {
        p1.death_timer = (p1.death_timer - dt).max(0.0);
        if p1.death_timer == 0.0 {
            let p2_body_copy = p2.body.clone();
            respawn_snake(
                &mut commands,
                &layout,
                &sprites,
                &mut p1,
                &[&p2_body_copy],
                &mut head_transforms,
                game_clock.tick_duration,
            );
            p1_just_respawned = true;
            notification.text = "Player 1 respawned!".to_string();
            notification.timer = Timer::from_seconds(2.0, TimerMode::Once);
        }
    }

    if p2.death_timer > 0.0 {
        p2.death_timer = (p2.death_timer - dt).max(0.0);
        if p2.death_timer == 0.0 {
            let p1_body_copy = p1.body.clone();
            respawn_snake(
                &mut commands,
                &layout,
                &sprites,
                &mut p2,
                &[&p1_body_copy],
                &mut head_transforms,
                game_clock.tick_duration,
            );
            p2_just_respawned = true;
            notification.text = "Player 2 respawned!".to_string();
            notification.timer = Timer::from_seconds(2.0, TimerMode::Once);
        }
    }

    // Decrement move timers for living snakes that did not just respawn on this frame
    if p1.death_timer == 0.0 && !p1_just_respawned {
        p1.move_timer -= dt;
    }
    if p2.death_timer == 0.0 && !p2_just_respawned {
        p2.move_timer -= dt;
    }

    let p1_ready = p1.death_timer == 0.0 && !p1_just_respawned && p1.move_timer <= 0.0;
    let p2_ready = p2.death_timer == 0.0 && !p2_just_respawned && p2.move_timer <= 0.0;

    if !p1_ready && !p2_ready {
        return;
    }

    // Reset move timers for whichever snakes are stepping
    if p1_ready {
        let p1_step_interval = if p1.boost_timer > 0.0 {
            game_clock.tick_duration * 0.5
        } else {
            game_clock.tick_duration
        };
        p1.move_timer = p1_step_interval;
    }

    if p2_ready {
        let p2_step_interval = if p2.boost_timer > 0.0 {
            game_clock.tick_duration * 0.5
        } else {
            game_clock.tick_duration
        };
        p2.move_timer = p2_step_interval;
    }

    let p1_head = p1.body[0];
    let p2_head = p2.body[0];

    let p1_next = p1_head + p1.next_direction.to_ivec2();
    let p2_next = p2_head + p2.next_direction.to_ivec2();

    // Collision detection depending on which snakes are stepping
    let mut p1_dies = false;
    let mut p2_dies = false;

    if p1_ready && p2_ready {
        // Both stepping simultaneously
        let p1_wall = p1_next.x < 0 || p1_next.x >= GRID_WIDTH || p1_next.y < 0 || p1_next.y >= GRID_HEIGHT;
        let p2_wall = p2_next.x < 0 || p2_next.x >= GRID_WIDTH || p2_next.y < 0 || p2_next.y >= GRID_HEIGHT;

        let p1_body_len = p1.body.len();
        let p1_check_len = if p1.growth_pending == 0 && p1_body_len > 1 { p1_body_len - 1 } else { p1_body_len };
        let p1_self = p1.body.iter().take(p1_check_len).any(|&c| c == p1_next);

        let p2_body_len = p2.body.len();
        let p2_check_len = if p2.growth_pending == 0 && p2_body_len > 1 { p2_body_len - 1 } else { p2_body_len };
        let p2_self = p2.body.iter().take(p2_check_len).any(|&c| c == p2_next);

        let p1_hits_p2 = p2.body.iter().any(|&c| c == p1_next);
        let p2_hits_p1 = p1.body.iter().any(|&c| c == p2_next);

        let head_on = p1_next == p2_next;

        p1_dies = p1_wall || p1_self || p1_hits_p2 || head_on;
        p2_dies = p2_wall || p2_self || p2_hits_p1 || head_on;
    } else if p1_ready {
        // Only Player 1 stepping
        let p1_wall = p1_next.x < 0 || p1_next.x >= GRID_WIDTH || p1_next.y < 0 || p1_next.y >= GRID_HEIGHT;

        let p1_body_len = p1.body.len();
        let p1_check_len = if p1.growth_pending == 0 && p1_body_len > 1 { p1_body_len - 1 } else { p1_body_len };
        let p1_self = p1.body.iter().take(p1_check_len).any(|&c| c == p1_next);

        let p1_hits_p2 = p2.death_timer == 0.0 && p2.body.iter().any(|&c| c == p1_next);

        p1_dies = p1_wall || p1_self || p1_hits_p2;
    } else if p2_ready {
        // Only Player 2 stepping
        let p2_wall = p2_next.x < 0 || p2_next.x >= GRID_WIDTH || p2_next.y < 0 || p2_next.y >= GRID_HEIGHT;

        let p2_body_len = p2.body.len();
        let p2_check_len = if p2.growth_pending == 0 && p2_body_len > 1 { p2_body_len - 1 } else { p2_body_len };
        let p2_self = p2.body.iter().take(p2_check_len).any(|&c| c == p2_next);

        let p2_hits_p1 = p1.death_timer == 0.0 && p1.body.iter().any(|&c| c == p2_next);

        p2_dies = p2_wall || p2_self || p2_hits_p1;
    }

    // Handle snake deaths & life deductions
    let p1_dead = p1_ready && p1_dies;
    let p2_dead = p2_ready && p2_dies;

    if p1_dead {
        p1.lives = p1.lives.saturating_sub(1);
    }
    if p2_dead {
        p2.lives = p2.lives.saturating_sub(1);
    }

    if p1_dead || p2_dead {
        winner_info.p1_final_score = p1.score;
        winner_info.p2_final_score = p2.score;

        // If a snake dies and has no more lives, that player loses the round
        if p1.lives == 0 && p2.lives == 0 {
            winner_info.winner = None;
            winner_info.reason = "Both players died and have no more lives! The match ended in a Draw!".to_string();
            next_state.set(GameState::RoundOver);
            return;
        } else if p1.lives == 0 {
            winner_info.winner = Some(PlayerId::Player2);
            winner_info.reason = "Player 1 died and has no more lives! Player 2 wins the round!".to_string();
            next_state.set(GameState::RoundOver);
            return;
        } else if p2.lives == 0 {
            winner_info.winner = Some(PlayerId::Player1);
            winner_info.reason = "Player 2 died and has no more lives! Player 1 wins the round!".to_string();
            next_state.set(GameState::RoundOver);
            return;
        }

        // Both players still have lives remaining: start 1.0s death transition
        if p1_dead && p2_dead {
            p1.death_timer = 1.0;
            p1.boost_timer = 0.0;
            p2.death_timer = 1.0;
            p2.boost_timer = 0.0;
            notification.text = format!("Both snakes crashed! (P1: {} left, P2: {} left)", p1.lives, p2.lives);
            notification.timer = Timer::from_seconds(2.0, TimerMode::Once);
        } else if p1_dead {
            p1.death_timer = 1.0;
            p1.boost_timer = 0.0;
            let unit = if p1.lives == 1 { "life" } else { "lives" };
            notification.text = format!("Player 1 died! ({} {} remaining)", p1.lives, unit);
            notification.timer = Timer::from_seconds(2.0, TimerMode::Once);
        } else if p2_dead {
            p2.death_timer = 1.0;
            p2.boost_timer = 0.0;
            let unit = if p2.lives == 1 { "life" } else { "lives" };
            notification.text = format!("Player 2 died! ({} {} remaining)", p2.lives, unit);
            notification.timer = Timer::from_seconds(2.0, TimerMode::Once);
        }
    }

    // Collect fruits for lookup
    let fruits: Vec<(Entity, IVec2, FruitType)> = fruits_query.iter().map(|(e, f)| (e, f.cell, f.fruit_type)).collect();

    // Move surviving snakes
    // 1. Move Player 1 if alive and was ready
    if p1_ready && !p1_dies {
        p1.direction = p1.next_direction;
        let old_head = p1.body[0];

        // Check fruit
        let mut eaten_fruit = None;
        for &(fruit_entity, cell, fruit_type) in &fruits {
            if cell == p1_next {
                eaten_fruit = Some((fruit_entity, fruit_type));
                break;
            }
        }

        if let Some((fruit_entity, fruit_type)) = eaten_fruit {
            let points = fruit_type.points();
            p1.score += points;
            p1.growth_pending += fruit_type.growth();
            commands.entity(fruit_entity).despawn();

            let fruit_pos = layout.grid_to_world(p1_next);
            spawn_floating_score(&mut commands, fruit_pos, points, layout.cell_size);

            if fruit_type == FruitType::HotPepper {
                let old_peppers = p1.stored_peppers;
                p1.stored_peppers = (p1.stored_peppers + 1).min(3);
                if p1.stored_peppers > old_peppers {
                    notification.text = format!("P1 collected a Hot Pepper! ({}/3 stored - Press A to BOOST!)", p1.stored_peppers);
                } else {
                    notification.text = "P1 ate a Hot Pepper! (Stored: 3/3 Max)".to_string();
                }
            } else {
                notification.text = format!("P1 ate {}! (+{} length)", fruit_type.name(), fruit_type.growth());
            }
            notification.timer = Timer::from_seconds(2.5, TimerMode::Once);

            let mut occupied: Vec<IVec2> = p1.body.iter().chain(p2.body.iter()).copied().collect();
            let mut rng = rand::rng();
            spawn_one_fruit(&mut commands, &layout, &sprites, &mut occupied, 0.0, &mut rng);

            // Play fruit eating sound effect:
            // "coin.wav" for normal fruits, "SFX_Powerup_01.wav" for Hot Pepper
            let sound = sound_assets.fruit_sound(fruit_type);
            let volume = if fruit_type == FruitType::HotPepper {
                Volume::Linear(1.0)
            } else {
                Volume::Linear(1.4)
            };
            commands.spawn((
                AudioPlayer::new(sound),
                PlaybackSettings::DESPAWN.with_volume(volume),
            ));
        }

        p1.body.push_front(p1_next);

        let seg_size = (layout.cell_size * 1.5).max(3.0);

        if p1.growth_pending > 0 {
            p1.growth_pending -= 1;
            let seg_world = layout.grid_to_world(old_head);
            let new_seg_entity = commands.spawn((
                SnakeSegment { player: PlayerId::Player1, index: p1.body.len() },
                Sprite {
                    image: sprites.p1_body.clone(),
                    custom_size: Some(Vec2::splat(seg_size)),
                    ..default()
                },
                Transform::from_translation(seg_world.extend(1.0)),
            )).id();

            p1.segment_entities.push_front(new_seg_entity);
        } else {
            p1.body.pop_back();
            if let Some(tail_entity) = p1.segment_entities.pop_back() {
                if let Ok(mut transform) = segment_transforms.get_mut(tail_entity) {
                    transform.translation = layout.grid_to_world(old_head).extend(1.0);
                } else {
                    commands.entity(tail_entity).insert(Transform::from_translation(layout.grid_to_world(old_head).extend(1.0)));
                }
                p1.segment_entities.push_front(tail_entity);
            }
        }

        if let Ok(mut transform) = head_transforms.get_mut(p1.head_entity) {
            transform.translation = layout.grid_to_world(p1_next).extend(2.0);
            transform.rotation = Quat::from_rotation_z(p1.direction.angle());
        }
    }

    // 2. Move Player 2 if alive and was ready
    if p2_ready && !p2_dies {
        p2.direction = p2.next_direction;
        let old_head = p2.body[0];

        // Check fruit
        let mut eaten_fruit = None;
        for &(fruit_entity, cell, fruit_type) in &fruits {
            if cell == p2_next {
                eaten_fruit = Some((fruit_entity, fruit_type));
                break;
            }
        }

        if let Some((fruit_entity, fruit_type)) = eaten_fruit {
            let points = fruit_type.points();
            p2.score += points;
            p2.growth_pending += fruit_type.growth();
            commands.entity(fruit_entity).despawn();

            let fruit_pos = layout.grid_to_world(p2_next);
            spawn_floating_score(&mut commands, fruit_pos, points, layout.cell_size);

            if fruit_type == FruitType::HotPepper {
                let old_peppers = p2.stored_peppers;
                p2.stored_peppers = (p2.stored_peppers + 1).min(3);
                if p2.stored_peppers > old_peppers {
                    notification.text = format!("P2 collected a Hot Pepper! ({}/3 stored - Press A to BOOST!)", p2.stored_peppers);
                } else {
                    notification.text = "P2 ate a Hot Pepper! (Stored: 3/3 Max)".to_string();
                }
            } else {
                notification.text = format!("P2 ate {}! (+{} length)", fruit_type.name(), fruit_type.growth());
            }
            notification.timer = Timer::from_seconds(2.5, TimerMode::Once);

            let mut occupied: Vec<IVec2> = p1.body.iter().chain(p2.body.iter()).copied().collect();
            let mut rng = rand::rng();
            spawn_one_fruit(&mut commands, &layout, &sprites, &mut occupied, 0.0, &mut rng);

            // Play fruit eating sound effect:
            // "coin.wav" for normal fruits, "SFX_Powerup_01.wav" for Hot Pepper
            let sound = sound_assets.fruit_sound(fruit_type);
            let volume = if fruit_type == FruitType::HotPepper {
                Volume::Linear(1.0)
            } else {
                Volume::Linear(1.4)
            };
            commands.spawn((
                AudioPlayer::new(sound),
                PlaybackSettings::DESPAWN.with_volume(volume),
            ));
        }

        p2.body.push_front(p2_next);

        let seg_size = (layout.cell_size * 1.5).max(3.0);

        if p2.growth_pending > 0 {
            p2.growth_pending -= 1;
            let seg_world = layout.grid_to_world(old_head);
            let new_seg_entity = commands.spawn((
                SnakeSegment { player: PlayerId::Player2, index: p2.body.len() },
                Sprite {
                    image: sprites.p2_body.clone(),
                    custom_size: Some(Vec2::splat(seg_size)),
                    ..default()
                },
                Transform::from_translation(seg_world.extend(1.0)),
            )).id();

            p2.segment_entities.push_front(new_seg_entity);
        } else {
            p2.body.pop_back();
            if let Some(tail_entity) = p2.segment_entities.pop_back() {
                if let Ok(mut transform) = segment_transforms.get_mut(tail_entity) {
                    transform.translation = layout.grid_to_world(old_head).extend(1.0);
                } else {
                    commands.entity(tail_entity).insert(Transform::from_translation(layout.grid_to_world(old_head).extend(1.0)));
                }
                p2.segment_entities.push_front(tail_entity);
            }
        }

        if let Ok(mut transform) = head_transforms.get_mut(p2.head_entity) {
            transform.translation = layout.grid_to_world(p2_next).extend(2.0);
            transform.rotation = Quat::from_rotation_z(p2.direction.angle());
        }
    }
}

pub fn spawn_floating_score(
    commands: &mut Commands,
    world_pos: Vec2,
    points: u32,
    cell_size: f32,
) -> Entity {
    let font_size = (cell_size * 1.5).clamp(16.0, 24.0);
    let move_speed = (cell_size * 2.5).clamp(25.0, 45.0);
    let text_str = format!("+{}", points);

    // 8-directional outline offsets around the center (1.5 px radius) for a crisp solid border
    let outline_offsets = [
        Vec2::new(-1.5, 0.0),
        Vec2::new(1.5, 0.0),
        Vec2::new(0.0, -1.5),
        Vec2::new(0.0, 1.5),
        Vec2::new(-1.2, -1.2),
        Vec2::new(-1.2, 1.2),
        Vec2::new(1.2, -1.2),
        Vec2::new(1.2, 1.2),
    ];

    let mut outline_entities = Vec::with_capacity(outline_offsets.len());
    for offset in outline_offsets {
        let outline_pos = (world_pos + offset).extend(4.9);
        let id = commands.spawn((
            FloatingScoreOutline,
            Text2d::new(text_str.clone()),
            TextFont {
                font_size: FontSize::Px(font_size),
                weight: FontWeight::BOLD,
                ..default()
            },
            TextColor(COLOR_FLOATING_SCORE_OUTLINE),
            Transform::from_translation(outline_pos),
        )).id();
        outline_entities.push(id);
    }

    commands.spawn((
        FloatingScore {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            initial_color: COLOR_FLOATING_SCORE,
            move_speed,
            outline_entities,
        },
        Text2d::new(text_str),
        TextFont {
            font_size: FontSize::Px(font_size),
            weight: FontWeight::BOLD,
            ..default()
        },
        TextColor(COLOR_FLOATING_SCORE),
        Transform::from_translation(world_pos.extend(5.0)),
    )).id()
}

pub fn update_floating_scores(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut FloatingScore, &mut Transform, &mut TextColor)>,
    mut outline_query: Query<(&mut Transform, &mut TextColor), (With<FloatingScoreOutline>, Without<FloatingScore>)>,
) {
    let dt = time.delta_secs();
    for (entity, mut score, mut transform, mut color) in &mut query {
        score.timer.tick(time.delta());

        if score.timer.is_finished() {
            for &outline_id in &score.outline_entities {
                commands.entity(outline_id).despawn();
            }
            commands.entity(entity).despawn();
        } else {
            let dy = score.move_speed * dt;
            transform.translation.y += dy;

            let alpha = score.timer.fraction_remaining().clamp(0.0, 1.0);
            color.0 = score.initial_color.with_alpha(alpha);

            for &outline_id in &score.outline_entities {
                if let Ok((mut outline_tf, mut outline_col)) = outline_query.get_mut(outline_id) {
                    outline_tf.translation.y += dy;
                    outline_col.0 = Color::srgba(0.0, 0.0, 0.0, alpha);
                }
            }
        }
    }
}

pub fn cleanup_floating_scores(
    mut commands: Commands,
    scores_query: Query<Entity, With<FloatingScore>>,
    outlines_query: Query<Entity, With<FloatingScoreOutline>>,
) {
    for entity in &scores_query {
        commands.entity(entity).despawn();
    }
    for entity in &outlines_query {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_fruit_points_values() {
        assert_eq!(FruitType::Apple.points(), 1);
        assert_eq!(FruitType::Strawberry.points(), 1);
        assert_eq!(FruitType::Orange.points(), 2);
        assert_eq!(FruitType::Grapes.points(), 2);
        assert_eq!(FruitType::Banana.points(), 2);
        assert_eq!(FruitType::Watermelon.points(), 3);
        assert_eq!(FruitType::Starfruit.points(), 4);
        assert_eq!(FruitType::HotPepper.points(), 3);
    }

    #[test]
    fn test_spawn_floating_score() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let fruit_pos = Vec2::new(50.0, 75.0);
        let cell_size = 14.0;
        let points = 3;

        let entity = {
            let mut commands = app.world_mut().commands();
            let mut spawned_id = None;
            spawn_floating_score(&mut commands, fruit_pos, points, cell_size);
            // Run commands
            app.world_mut().flush();
            let mut query = app.world_mut().query::<(Entity, &FloatingScore, &Text2d, &Transform, &TextColor)>();
            for (e, score, text, transform, color) in query.iter(app.world()) {
                assert_eq!(text.0, "+3");
                assert_eq!(score.timer.duration(), Duration::from_secs_f32(1.0));
                assert!(!score.timer.is_finished());
                assert_eq!(transform.translation.x, 50.0);
                assert_eq!(transform.translation.y, 75.0);
                assert_eq!(transform.translation.z, 5.0);
                assert_eq!(color.0, COLOR_FLOATING_SCORE);
                assert_eq!(score.outline_entities.len(), 8);
                spawned_id = Some(e);
            }
            spawned_id.expect("FloatingScore entity should have been spawned")
        };

        assert!(app.world().get_entity(entity).is_ok());

        // Verify outline entities exist with black text
        let mut outline_query = app.world_mut().query::<(&FloatingScoreOutline, &Text2d, &TextColor)>();
        let mut count = 0;
        for (_outline, text, color) in outline_query.iter(app.world()) {
            assert_eq!(text.0, "+3");
            assert_eq!(color.0, COLOR_FLOATING_SCORE_OUTLINE);
            count += 1;
        }
        assert_eq!(count, 8);
    }

    #[test]
    fn test_update_floating_scores_movement_and_fade() {
        let mut app = App::new();
        let mut time: Time<()> = Time::default();
        time.advance_by(Duration::from_millis(500));
        app.insert_resource(time);

        let outline_id = app.world_mut().spawn((
            FloatingScoreOutline,
            Text2d::new("+2"),
            TextColor(COLOR_FLOATING_SCORE_OUTLINE),
            Transform::from_translation(Vec3::new(10.0, 20.0, 4.9)),
        )).id();

        let entity = app.world_mut().spawn((
            FloatingScore {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
                initial_color: COLOR_FLOATING_SCORE,
                move_speed: 40.0,
                outline_entities: vec![outline_id],
            },
            Text2d::new("+2"),
            TextColor(COLOR_FLOATING_SCORE),
            Transform::from_translation(Vec3::new(10.0, 20.0, 5.0)),
        )).id();

        // Run system for first 0.5s step
        let _ = app.world_mut().run_system_cached(update_floating_scores);

        // Verify entity moved up and faded
        {
            let transform = app.world().get::<Transform>(entity).unwrap();
            let color = app.world().get::<TextColor>(entity).unwrap();
            let score = app.world().get::<FloatingScore>(entity).unwrap();
            let outline_transform = app.world().get::<Transform>(outline_id).unwrap();
            let outline_color = app.world().get::<TextColor>(outline_id).unwrap();

            // Started at y=20.0, moved up at 40.0 units/sec for 0.5s => y should be 40.0
            assert_eq!(transform.translation.y, 40.0);
            assert_eq!(outline_transform.translation.y, 40.0);

            // Timer at 0.5s should have 0.5 fraction remaining
            let frac = score.timer.fraction_remaining();
            assert!((frac - 0.5).abs() < 0.001);
            assert_eq!(color.0, COLOR_FLOATING_SCORE.with_alpha(frac));
            assert_eq!(outline_color.0, Color::srgba(0.0, 0.0, 0.0, frac));
        }

        // Advance another 0.6s (total 1.1s > 1.0s)
        app.world_mut().resource_mut::<Time>().advance_by(Duration::from_millis(600));
        let _ = app.world_mut().run_system_cached(update_floating_scores);

        // Both entity and outline should be despawned after 1.0 second
        assert!(app.world().get_entity(entity).is_err());
        assert!(app.world().get_entity(outline_id).is_err());
    }

    #[test]
    fn test_cleanup_floating_scores() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, cleanup_floating_scores);

        let o1 = app.world_mut().spawn((
            FloatingScoreOutline,
            Text2d::new("+1"),
        )).id();

        let e1 = app.world_mut().spawn((
            FloatingScore {
                timer: Timer::from_seconds(1.0, TimerMode::Once),
                initial_color: COLOR_FLOATING_SCORE,
                move_speed: 30.0,
                outline_entities: vec![o1],
            },
            Text2d::new("+1"),
        )).id();

        assert!(app.world().get_entity(e1).is_ok());
        assert!(app.world().get_entity(o1).is_ok());

        app.update();

        assert!(app.world().get_entity(e1).is_err());
        assert!(app.world().get_entity(o1).is_err());
    }

    #[test]
    fn test_fruit_eating_spawns_floating_score() {
        let mut app = App::new();
        let mut time: Time<()> = Time::default();
        time.advance_by(Duration::from_millis(200));
        app.insert_resource(time);

        let mut layout = ArenaLayout::default();
        layout.cell_size = 15.0;
        app.insert_resource(layout);
        app.insert_resource(SpriteAssets::default());
        app.insert_resource(SoundAssets {
            coin: Handle::default(),
            powerup: Handle::default(),
            background_music: Handle::default(),
        });
        app.insert_resource(GameClock::default());
        app.insert_resource(NotificationMessage::default());
        app.insert_resource(GameWinner::default());
        app.add_plugins(bevy::state::app::StatesPlugin);
        app.init_state::<GameState>();

        // Spawn P1 head & body
        let p1_head = app.world_mut().spawn((
            SnakeHead { player: PlayerId::Player1 },
            Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        )).id();

        let mut p1_body = std::collections::VecDeque::new();
        p1_body.push_back(IVec2::new(10, 10));
        p1_body.push_back(IVec2::new(9, 10));

        app.world_mut().spawn(SnakeState {
            player: PlayerId::Player1,
            direction: Direction::Right,
            next_direction: Direction::Right,
            body: p1_body,
            segment_entities: std::collections::VecDeque::new(),
            growth_pending: 0,
            score: 0,
            respawns: 0,
            lives: INITIAL_LIVES,
            head_entity: p1_head,
            stored_peppers: 0,
            boost_timer: 0.0,
            move_timer: 0.0,
            death_timer: 0.0,
        });

        // Spawn P2 head & body far away
        let p2_head = app.world_mut().spawn((
            SnakeHead { player: PlayerId::Player2 },
            Transform::from_translation(Vec3::new(100.0, 100.0, 2.0)),
        )).id();

        let mut p2_body = std::collections::VecDeque::new();
        p2_body.push_back(IVec2::new(50, 50));
        p2_body.push_back(IVec2::new(50, 49));

        app.world_mut().spawn(SnakeState {
            player: PlayerId::Player2,
            direction: Direction::Up,
            next_direction: Direction::Up,
            body: p2_body,
            segment_entities: std::collections::VecDeque::new(),
            growth_pending: 0,
            score: 0,
            respawns: 0,
            lives: INITIAL_LIVES,
            head_entity: p2_head,
            stored_peppers: 0,
            boost_timer: 0.0,
            move_timer: 1.0, // not ready
            death_timer: 0.0,
        });

        // Spawn a Starfruit (4 points) right at P1's next cell (11, 10)
        let fruit_entity = app.world_mut().spawn((
            Fruit {
                fruit_type: FruitType::Starfruit,
                cell: IVec2::new(11, 10),
                phase_offset: 0.0,
            },
            Transform::default(),
        )).id();

        // Run game_clock_tick
        let _ = app.world_mut().run_system_cached(game_clock_tick);

        // Verify fruit entity is despawned
        assert!(app.world().get_entity(fruit_entity).is_err());

        // Verify P1 score increased by 4
        let mut snakes_query = app.world_mut().query::<&SnakeState>();
        for snake in snakes_query.iter(app.world()) {
            if snake.player == PlayerId::Player1 {
                assert_eq!(snake.score, 4);
            }
        }

        // Verify FloatingScore entity was spawned with "+4" and COLOR_FLOATING_SCORE
        let mut score_query = app.world_mut().query::<(&FloatingScore, &Text2d, &TextColor)>();
        let mut found_floating_score = false;
        for (score, text, color) in score_query.iter(app.world()) {
            assert_eq!(text.0, "+4");
            assert_eq!(color.0, COLOR_FLOATING_SCORE);
            assert_eq!(score.timer.duration(), Duration::from_secs_f32(1.0));
            assert_eq!(score.outline_entities.len(), 8);
            found_floating_score = true;
        }
        assert!(found_floating_score, "Floating score should be spawned when fruit is eaten");

        // Verify 8 outline entities with black color exist
        let mut outline_query = app.world_mut().query::<(&FloatingScoreOutline, &Text2d, &TextColor)>();
        let mut outline_count = 0;
        for (_outline, text, color) in outline_query.iter(app.world()) {
            assert_eq!(text.0, "+4");
            assert_eq!(color.0, COLOR_FLOATING_SCORE_OUTLINE);
            outline_count += 1;
        }
        assert_eq!(outline_count, 8);
    }

    #[test]
    fn test_snake_initial_lives() {
        let mut app = App::new();
        let mut layout = ArenaLayout::default();
        layout.cell_size = 15.0;
        let sprites = SpriteAssets::default();

        let entity = crate::snake::spawn_snake(
            &mut app.world_mut().commands(),
            &layout,
            &sprites,
            PlayerId::Player1,
            IVec2::new(10, 10),
            Direction::Right,
        );
        app.world_mut().flush();

        let snake = app.world().get::<SnakeState>(entity).unwrap();
        assert_eq!(snake.lives, 3);
        assert_eq!(INITIAL_LIVES, 3);
    }

    #[test]
    fn test_snake_loses_one_life_on_death() {
        let mut app = App::new();
        let mut time: Time<()> = Time::default();
        time.advance_by(Duration::from_millis(200));
        app.insert_resource(time);

        let mut layout = ArenaLayout::default();
        layout.cell_size = 15.0;
        app.insert_resource(layout);
        app.insert_resource(SpriteAssets::default());
        app.insert_resource(SoundAssets {
            coin: Handle::default(),
            powerup: Handle::default(),
            background_music: Handle::default(),
        });
        app.insert_resource(GameClock::default());
        app.insert_resource(NotificationMessage::default());
        app.insert_resource(GameWinner::default());
        app.add_plugins(bevy::state::app::StatesPlugin);
        app.init_state::<GameState>();

        // P1 moving Left towards wall at x=0
        let p1_head = app.world_mut().spawn((
            SnakeHead { player: PlayerId::Player1 },
            Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        )).id();
        let mut p1_body = std::collections::VecDeque::new();
        p1_body.push_back(IVec2::new(0, 10)); // Next step is (-1, 10) => hits wall!
        p1_body.push_back(IVec2::new(1, 10));

        let p1_entity = app.world_mut().spawn(SnakeState {
            player: PlayerId::Player1,
            direction: Direction::Left,
            next_direction: Direction::Left,
            body: p1_body,
            segment_entities: std::collections::VecDeque::new(),
            growth_pending: 0,
            score: 0,
            respawns: 0,
            lives: 3,
            head_entity: p1_head,
            stored_peppers: 0,
            boost_timer: 0.0,
            move_timer: 0.0,
            death_timer: 0.0,
        }).id();

        // P2 far away not ready
        let p2_head = app.world_mut().spawn((
            SnakeHead { player: PlayerId::Player2 },
            Transform::from_translation(Vec3::new(100.0, 100.0, 2.0)),
        )).id();
        let mut p2_body = std::collections::VecDeque::new();
        p2_body.push_back(IVec2::new(50, 50));
        p2_body.push_back(IVec2::new(50, 49));

        app.world_mut().spawn(SnakeState {
            player: PlayerId::Player2,
            direction: Direction::Up,
            next_direction: Direction::Up,
            body: p2_body,
            segment_entities: std::collections::VecDeque::new(),
            growth_pending: 0,
            score: 0,
            respawns: 0,
            lives: 3,
            head_entity: p2_head,
            stored_peppers: 0,
            boost_timer: 0.0,
            move_timer: 1.0,
            death_timer: 0.0,
        });

        // Run game_clock_tick
        let _ = app.world_mut().run_system_cached(game_clock_tick);

        let p1 = app.world().get::<SnakeState>(p1_entity).unwrap();
        // P1 should have lost 1 life: 3 -> 2
        assert_eq!(p1.lives, 2);
        assert_eq!(p1.death_timer, 1.0);

        // Game should NOT be over yet
        let next_state = app.world().resource::<NextState<GameState>>();
        assert!(matches!(next_state, NextState::Unchanged));
    }

    #[test]
    fn test_snake_dies_with_no_more_lives_loses_round() {
        let mut app = App::new();
        let mut time: Time<()> = Time::default();
        time.advance_by(Duration::from_millis(200));
        app.insert_resource(time);

        let mut layout = ArenaLayout::default();
        layout.cell_size = 15.0;
        app.insert_resource(layout);
        app.insert_resource(SpriteAssets::default());
        app.insert_resource(SoundAssets {
            coin: Handle::default(),
            powerup: Handle::default(),
            background_music: Handle::default(),
        });
        app.insert_resource(GameClock::default());
        app.insert_resource(NotificationMessage::default());
        app.insert_resource(GameWinner::default());
        app.add_plugins(bevy::state::app::StatesPlugin);
        app.init_state::<GameState>();

        // P1 moving Left towards wall at x=0 with only 1 life left
        let p1_head = app.world_mut().spawn((
            SnakeHead { player: PlayerId::Player1 },
            Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        )).id();
        let mut p1_body = std::collections::VecDeque::new();
        p1_body.push_back(IVec2::new(0, 10)); // Next step is (-1, 10) => hits wall!
        p1_body.push_back(IVec2::new(1, 10));

        let p1_entity = app.world_mut().spawn(SnakeState {
            player: PlayerId::Player1,
            direction: Direction::Left,
            next_direction: Direction::Left,
            body: p1_body,
            segment_entities: std::collections::VecDeque::new(),
            growth_pending: 0,
            score: 5,
            respawns: 2,
            lives: 1, // Only 1 life remaining!
            head_entity: p1_head,
            stored_peppers: 0,
            boost_timer: 0.0,
            move_timer: 0.0,
            death_timer: 0.0,
        }).id();

        // P2 far away not ready
        let p2_head = app.world_mut().spawn((
            SnakeHead { player: PlayerId::Player2 },
            Transform::from_translation(Vec3::new(100.0, 100.0, 2.0)),
        )).id();
        let mut p2_body = std::collections::VecDeque::new();
        p2_body.push_back(IVec2::new(50, 50));
        p2_body.push_back(IVec2::new(50, 49));

        app.world_mut().spawn(SnakeState {
            player: PlayerId::Player2,
            direction: Direction::Up,
            next_direction: Direction::Up,
            body: p2_body,
            segment_entities: std::collections::VecDeque::new(),
            growth_pending: 0,
            score: 8,
            respawns: 0,
            lives: 3,
            head_entity: p2_head,
            stored_peppers: 0,
            boost_timer: 0.0,
            move_timer: 1.0,
            death_timer: 0.0,
        });

        // Run game_clock_tick
        let _ = app.world_mut().run_system_cached(game_clock_tick);

        let p1 = app.world().get::<SnakeState>(p1_entity).unwrap();
        // P1 has 0 lives remaining
        assert_eq!(p1.lives, 0);

        // Player 1 lost the round => Player 2 wins!
        let winner = app.world().resource::<GameWinner>();
        assert_eq!(winner.winner, Some(PlayerId::Player2));
        assert!(winner.reason.contains("Player 1 died and has no more lives"));

        let next_state = app.world().resource::<NextState<GameState>>();
        assert!(matches!(next_state, NextState::Pending(GameState::RoundOver)));
    }

    #[test]
    fn test_both_snakes_die_with_no_more_lives_draw() {
        let mut app = App::new();
        let mut time: Time<()> = Time::default();
        time.advance_by(Duration::from_millis(200));
        app.insert_resource(time);

        let mut layout = ArenaLayout::default();
        layout.cell_size = 15.0;
        app.insert_resource(layout);
        app.insert_resource(SpriteAssets::default());
        app.insert_resource(SoundAssets {
            coin: Handle::default(),
            powerup: Handle::default(),
            background_music: Handle::default(),
        });
        app.insert_resource(GameClock::default());
        app.insert_resource(NotificationMessage::default());
        app.insert_resource(GameWinner::default());
        app.add_plugins(bevy::state::app::StatesPlugin);
        app.init_state::<GameState>();

        // P1 at (10, 10) facing Right, P2 at (12, 10) facing Left => head on at (11, 10)
        let p1_head = app.world_mut().spawn((
            SnakeHead { player: PlayerId::Player1 },
            Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
        )).id();
        let mut p1_body = std::collections::VecDeque::new();
        p1_body.push_back(IVec2::new(10, 10));
        p1_body.push_back(IVec2::new(9, 10));

        let p1_entity = app.world_mut().spawn(SnakeState {
            player: PlayerId::Player1,
            direction: Direction::Right,
            next_direction: Direction::Right,
            body: p1_body,
            segment_entities: std::collections::VecDeque::new(),
            growth_pending: 0,
            score: 2,
            respawns: 2,
            lives: 1, // 1 life left
            head_entity: p1_head,
            stored_peppers: 0,
            boost_timer: 0.0,
            move_timer: 0.0,
            death_timer: 0.0,
        }).id();

        let p2_head = app.world_mut().spawn((
            SnakeHead { player: PlayerId::Player2 },
            Transform::from_translation(Vec3::new(100.0, 100.0, 2.0)),
        )).id();
        let mut p2_body = std::collections::VecDeque::new();
        p2_body.push_back(IVec2::new(12, 10));
        p2_body.push_back(IVec2::new(13, 10));

        let p2_entity = app.world_mut().spawn(SnakeState {
            player: PlayerId::Player2,
            direction: Direction::Left,
            next_direction: Direction::Left,
            body: p2_body,
            segment_entities: std::collections::VecDeque::new(),
            growth_pending: 0,
            score: 2,
            respawns: 2,
            lives: 1, // 1 life left
            head_entity: p2_head,
            stored_peppers: 0,
            boost_timer: 0.0,
            move_timer: 0.0,
            death_timer: 0.0,
        }).id();

        // Run game_clock_tick: head-on collision
        let _ = app.world_mut().run_system_cached(game_clock_tick);

        let p1 = app.world().get::<SnakeState>(p1_entity).unwrap();
        let p2 = app.world().get::<SnakeState>(p2_entity).unwrap();
        assert_eq!(p1.lives, 0);
        assert_eq!(p2.lives, 0);

        let winner = app.world().resource::<GameWinner>();
        assert_eq!(winner.winner, None); // Draw!
        assert!(winner.reason.contains("Draw"));

        let next_state = app.world().resource::<NextState<GameState>>();
        assert!(matches!(next_state, NextState::Pending(GameState::RoundOver)));
    }
}
