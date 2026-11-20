use bevy::prelude::*;
use std::collections::VecDeque;
use rand::RngExt;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;
use crate::svg_utils::SpriteAssets;

pub fn spawn_initial_snakes(
    mut commands: Commands,
    layout: Res<ArenaLayout>,
    sprites: Res<SpriteAssets>,
) {
    // 1. Spawn Player 1 (Cyan)
    spawn_snake(
        &mut commands,
        &layout,
        &sprites,
        PlayerId::Player1,
        IVec2::new(18, GRID_HEIGHT / 2),
        Direction::Right,
    );

    // 2. Spawn Player 2 (Orange)
    spawn_snake(
        &mut commands,
        &layout,
        &sprites,
        PlayerId::Player2,
        IVec2::new(GRID_WIDTH - 19, GRID_HEIGHT / 2),
        Direction::Left,
    );
}

pub fn spawn_snake(
    commands: &mut Commands,
    layout: &ArenaLayout,
    sprites: &SpriteAssets,
    player: PlayerId,
    start_pos: IVec2,
    direction: Direction,
) -> Entity {
    let (head_image, body_image) = match player {
        PlayerId::Player1 => (sprites.p1_head.clone(), sprites.p1_body.clone()),
        PlayerId::Player2 => (sprites.p2_head.clone(), sprites.p2_body.clone()),
    };

    let head_world = layout.grid_to_world(start_pos);
    let head_size = (layout.cell_size * 2.2).max(4.0);
    let seg_size = (layout.cell_size * 1.5).max(3.0);

    // Build initial body segments trailing behind start_pos
    let mut body = VecDeque::new();
    let mut segment_entities = VecDeque::new();

    body.push_back(start_pos);

    let step_back = direction.opposite().to_ivec2();
    for i in 1..INITIAL_SNAKE_LENGTH {
        let seg_cell = start_pos + step_back * (i as i32);
        body.push_back(seg_cell);

        let seg_world = layout.grid_to_world(seg_cell);
        let seg_entity = commands.spawn((
            SnakeSegment { player, index: i },
            Sprite {
                image: body_image.clone(),
                custom_size: Some(Vec2::splat(seg_size)),
                ..default()
            },
            Transform::from_translation(seg_world.extend(1.0)),
        )).id();

        segment_entities.push_back(seg_entity);
    }

    // Spawn Head entity using vector SVG texture
    // Designed pointing Right (0 deg), so rotation by direction.angle() naturally faces user's direction
    let head_entity = commands.spawn((
        SnakeHead { player },
        Sprite {
            image: head_image,
            custom_size: Some(Vec2::splat(head_size)),
            ..default()
        },
        Transform {
            translation: head_world.extend(2.0),
            rotation: Quat::from_rotation_z(direction.angle()),
            ..default()
        },
    )).id();

    // Spawn SnakeState entity
    commands.spawn(SnakeState {
        player,
        direction,
        next_direction: direction,
        body,
        segment_entities,
        growth_pending: 0,
        score: 0,
        respawns: 0,
        lives: INITIAL_LIVES,
        head_entity,
        stored_peppers: 0,
        boost_timer: 0.0,
        move_timer: DEFAULT_TICK_SECONDS,
        death_timer: 0.0,
    }).id()
}

/// Updates head rotation IMMEDIATELY whenever the player changes direction,
/// fulfilling the requirement:
/// "As soon as the player presses a direction, the head of the snake should be
///  looking in the direction that the user pressed, so that the user can see
///  where the snake will be moving."
pub fn update_snake_head_visuals(
    snakes: Query<&SnakeState>,
    mut head_transforms: Query<&mut Transform, With<SnakeHead>>,
) {
    for snake in &snakes {
        if snake.death_timer > 0.0 {
            continue;
        }
        if let Ok(mut transform) = head_transforms.get_mut(snake.head_entity) {
            transform.rotation = Quat::from_rotation_z(snake.next_direction.angle());
        }
    }
}

/// Respawn a snake in an empty, safe location on the grid
pub fn respawn_snake<F: bevy::ecs::query::QueryFilter>(
    commands: &mut Commands,
    layout: &ArenaLayout,
    sprites: &SpriteAssets,
    snake: &mut SnakeState,
    other_snakes_bodies: &[&VecDeque<IVec2>],
    head_transforms: &mut Query<&mut Transform, F>,
    step_interval: f32,
) {
    snake.respawns += 1;

    // Despawn all existing body segments
    for entity in snake.segment_entities.drain(..) {
        commands.entity(entity).despawn();
    }
    snake.body.clear();
    snake.growth_pending = 0;
    snake.boost_timer = 0.0;
    // Set move_timer to a full step interval so the snake pauses upon respawning,
    // ensuring all newly spawned segment entities are fully initialized in the ECS world.
    snake.move_timer = step_interval;
    snake.death_timer = 0.0;

    let mut rng = rand::rng();

    // Find a suitable empty location with space to move
    let mut best_pos = IVec2::new(
        rng.random_range(14..(GRID_WIDTH - 14)),
        rng.random_range(12..(GRID_HEIGHT - 12)),
    );
    let mut best_dir = Direction::Right;

    for _ in 0..60 {
        let candidate_x = rng.random_range(10..(GRID_WIDTH - 10));
        let candidate_y = rng.random_range(8..(GRID_HEIGHT - 8));
        let candidate = IVec2::new(candidate_x, candidate_y);

        let dirs = [Direction::Right, Direction::Left, Direction::Up, Direction::Down];
        let dir = dirs[rng.random_range(0..4)];

        // Check clearance for candidate, trailing body cells, and forward path
        let mut clear = true;
        for step in 0..INITIAL_SNAKE_LENGTH as i32 {
            // Check trailing body cells (where the body segments will spawn)
            let seg_cell = candidate + dir.opposite().to_ivec2() * step;
            if seg_cell.x < 3 || seg_cell.x >= GRID_WIDTH - 3 || seg_cell.y < 3 || seg_cell.y >= GRID_HEIGHT - 3 {
                clear = false;
                break;
            }

            for other_body in other_snakes_bodies {
                if other_body.contains(&seg_cell) {
                    clear = false;
                    break;
                }
            }
            if !clear {
                break;
            }

            // Check forward path cells (where the snake will move)
            let fwd_cell = candidate + dir.to_ivec2() * step;
            if fwd_cell.x < 3 || fwd_cell.x >= GRID_WIDTH - 3 || fwd_cell.y < 3 || fwd_cell.y >= GRID_HEIGHT - 3 {
                clear = false;
                break;
            }

            for other_body in other_snakes_bodies {
                if other_body.contains(&fwd_cell) {
                    clear = false;
                    break;
                }
            }
            if !clear {
                break;
            }
        }

        if clear {
            best_pos = candidate;
            best_dir = dir;
            break;
        }
    }

    snake.direction = best_dir;
    snake.next_direction = best_dir;
    snake.body.push_back(best_pos);

    let body_image = match snake.player {
        PlayerId::Player1 => sprites.p1_body.clone(),
        PlayerId::Player2 => sprites.p2_body.clone(),
    };
    let seg_size = (layout.cell_size * 1.5).max(3.0);

    // Spawn new body segments trailing behind respawn position
    let step_back = best_dir.opposite().to_ivec2();
    for i in 1..INITIAL_SNAKE_LENGTH {
        let seg_cell = best_pos + step_back * (i as i32);
        snake.body.push_back(seg_cell);

        let seg_world = layout.grid_to_world(seg_cell);
        let seg_entity = commands.spawn((
            SnakeSegment { player: snake.player, index: i },
            Sprite {
                image: body_image.clone(),
                custom_size: Some(Vec2::splat(seg_size)),
                ..default()
            },
            Transform::from_translation(seg_world.extend(1.0)),
        )).id();

        snake.segment_entities.push_back(seg_entity);
    }

    // Move head transform to new spawn location and rotate facing direction
    if let Ok(mut transform) = head_transforms.get_mut(snake.head_entity) {
        transform.translation = layout.grid_to_world(best_pos).extend(2.0);
        transform.rotation = Quat::from_rotation_z(best_dir.angle());
    }
}

pub fn cleanup_snakes(
    mut commands: Commands,
    snakes: Query<(Entity, &SnakeState)>,
    segments: Query<Entity, With<SnakeSegment>>,
    heads: Query<Entity, With<SnakeHead>>,
) {
    for (entity, _) in &snakes {
        commands.entity(entity).despawn();
    }
    for entity in &segments {
        commands.entity(entity).despawn();
    }
    for entity in &heads {
        commands.entity(entity).despawn();
    }
}

/// Visual effects for snakes:
/// - Death transition: 1.0 second total duration (stops moving, turns grey, fades out to 0 alpha, then respawns).
/// - Pepper speed boost: Rapid flashing strobe effect (approx 10 Hz) while active.
/// - Normal: Resets head and body segments to normal color textures and Color::WHITE.
pub fn update_snake_boost_flashing(
    time: Res<Time>,
    sprites: Res<SpriteAssets>,
    snakes: Query<&SnakeState>,
    mut head_sprites: Query<&mut Sprite, (With<SnakeHead>, Without<SnakeSegment>)>,
    mut segment_sprites: Query<&mut Sprite, (With<SnakeSegment>, Without<SnakeHead>)>,
) {
    let t = time.elapsed_secs();
    // Fast pulsating strobe effect (approx 10 Hz)
    let flash_phase = (t * 22.0).sin();

    for snake in &snakes {
        if snake.death_timer > 0.0 {
            // Death transition (1.0 second total duration):
            // 1. Snake stops moving (handled in game_tick / input)
            // 2. Turns grey (0.2s - 0.5s)
            // 3. Fades out to 0 alpha (0.5s - 1.0s)
            // 4. Respawns at 1.0s (handled in game_tick)
            let elapsed = (1.0 - snake.death_timer).clamp(0.0, 1.0);
            let is_gray = elapsed >= 0.2;

            let alpha = if elapsed >= 0.5 {
                ((1.0 - elapsed) / 0.5).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let head_img = sprites.snake_head(snake.player, is_gray);
            let body_img = sprites.snake_body(snake.player, is_gray);
            let tint = if is_gray {
                Color::srgba(0.85, 0.85, 0.85, alpha)
            } else {
                Color::srgba(1.0, 1.0, 1.0, alpha)
            };

            if let Ok(mut sprite) = head_sprites.get_mut(snake.head_entity) {
                sprite.image = head_img;
                sprite.color = tint;
            }

            for &seg_entity in &snake.segment_entities {
                if let Ok(mut sprite) = segment_sprites.get_mut(seg_entity) {
                    sprite.image = body_img.clone();
                    sprite.color = tint;
                }
            }
        } else {
            let is_boosted = snake.boost_timer > 0.0;

            let tint = if is_boosted {
                if flash_phase > 0.0 {
                    // Hot glowing pepper flame tint
                    Color::srgb(2.2, 0.7, 0.1)
                } else {
                    // Super-charged golden white flash
                    Color::srgb(2.0, 2.0, 0.9)
                }
            } else {
                // Normal looks
                Color::WHITE
            };

            let head_img = sprites.snake_head(snake.player, false);
            let body_img = sprites.snake_body(snake.player, false);

            if let Ok(mut sprite) = head_sprites.get_mut(snake.head_entity) {
                sprite.image = head_img;
                sprite.color = tint;
            }

            for &seg_entity in &snake.segment_entities {
                if let Ok(mut sprite) = segment_sprites.get_mut(seg_entity) {
                    sprite.image = body_img.clone();
                    sprite.color = tint;
                }
            }
        }
    }
}
