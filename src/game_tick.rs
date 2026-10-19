use bevy::prelude::*;
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

    // 3. Tick game movement clock
    game_clock.tick_timer.tick(time.delta());
    if !game_clock.tick_timer.just_finished() {
        return;
    }

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

    let p1_head = p1.body[0];
    let p2_head = p2.body[0];

    let p1_next = p1_head + p1.next_direction.to_ivec2();
    let p2_next = p2_head + p2.next_direction.to_ivec2();

    // Check bounds / wall hits
    let p1_wall = p1_next.x < 0 || p1_next.x >= GRID_WIDTH || p1_next.y < 0 || p1_next.y >= GRID_HEIGHT;
    let p2_wall = p2_next.x < 0 || p2_next.x >= GRID_WIDTH || p2_next.y < 0 || p2_next.y >= GRID_HEIGHT;

    // Check self collisions (ignore tail if moving away)
    let p1_body_len = p1.body.len();
    let p1_check_len = if p1.growth_pending == 0 && p1_body_len > 1 { p1_body_len - 1 } else { p1_body_len };
    let p1_self = p1.body.iter().take(p1_check_len).any(|&c| c == p1_next);

    let p2_body_len = p2.body.len();
    let p2_check_len = if p2.growth_pending == 0 && p2_body_len > 1 { p2_body_len - 1 } else { p2_body_len };
    let p2_self = p2.body.iter().take(p2_check_len).any(|&c| c == p2_next);

    // Check opponent collisions
    let p1_hits_p2 = p2.body.iter().any(|&c| c == p1_next);
    let p2_hits_p1 = p1.body.iter().any(|&c| c == p2_next);

    // Head-to-head collision
    let head_on = p1_next == p2_next;

    let p1_dies = p1_wall || p1_self || p1_hits_p2 || head_on;
    let p2_dies = p2_wall || p2_self || p2_hits_p1 || head_on;

    // Handle Player 1 Death / Respawn
    if p1_dies {
        let p2_body_copy = p2.body.clone();
        respawn_snake(
            &mut commands,
            &layout,
            &sprites,
            &mut p1,
            &[&p2_body_copy],
            &mut head_transforms,
        );
        notification.text = "Player 1 died and respawned!".to_string();
        notification.timer = Timer::from_seconds(2.5, TimerMode::Once);
    }

    // Handle Player 2 Death / Respawn
    if p2_dies {
        let p1_body_copy = p1.body.clone();
        respawn_snake(
            &mut commands,
            &layout,
            &sprites,
            &mut p2,
            &[&p1_body_copy],
            &mut head_transforms,
        );
        notification.text = "Player 2 died and respawned!".to_string();
        notification.timer = Timer::from_seconds(2.5, TimerMode::Once);
    }

    // Collect fruits for lookup
    let fruits: Vec<(Entity, IVec2, FruitType)> = fruits_query.iter().map(|(e, f)| (e, f.cell, f.fruit_type)).collect();

    // Move surviving snakes
    // 1. Move Player 1 if alive
    if !p1_dies {
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
            p1.score += fruit_type.points();
            p1.growth_pending += fruit_type.growth();
            commands.entity(fruit_entity).despawn();

            notification.text = format!("P1 ate {}! (+{} length)", fruit_type.name(), fruit_type.growth());
            notification.timer = Timer::from_seconds(2.5, TimerMode::Once);

            let mut occupied: Vec<IVec2> = p1.body.iter().chain(p2.body.iter()).copied().collect();
            let mut rng = rand::rng();
            spawn_one_fruit(&mut commands, &layout, &sprites, &mut occupied, 0.0, &mut rng);
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
                }
                p1.segment_entities.push_front(tail_entity);
            }
        }

        if let Ok(mut transform) = head_transforms.get_mut(p1.head_entity) {
            transform.translation = layout.grid_to_world(p1_next).extend(2.0);
            transform.rotation = Quat::from_rotation_z(p1.direction.angle());
        }
    }

    // 2. Move Player 2 if alive
    if !p2_dies {
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
            p2.score += fruit_type.points();
            p2.growth_pending += fruit_type.growth();
            commands.entity(fruit_entity).despawn();

            notification.text = format!("P2 ate {}! (+{} length)", fruit_type.name(), fruit_type.growth());
            notification.timer = Timer::from_seconds(2.5, TimerMode::Once);

            let mut occupied: Vec<IVec2> = p1.body.iter().chain(p2.body.iter()).copied().collect();
            let mut rng = rand::rng();
            spawn_one_fruit(&mut commands, &layout, &sprites, &mut occupied, 0.0, &mut rng);
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
