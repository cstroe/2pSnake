use bevy::prelude::*;
use std::collections::VecDeque;
use rand::RngExt;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;

pub fn spawn_initial_snakes(
    mut commands: Commands,
    layout: Res<ArenaLayout>,
) {
    // 1. Spawn Player 1 (Cyan)
    spawn_snake(
        &mut commands,
        &layout,
        PlayerId::Player1,
        IVec2::new(25, GRID_HEIGHT / 2),
        Direction::Right,
    );

    // 2. Spawn Player 2 (Orange)
    spawn_snake(
        &mut commands,
        &layout,
        PlayerId::Player2,
        IVec2::new(GRID_WIDTH - 26, GRID_HEIGHT / 2),
        Direction::Left,
    );
}

pub fn spawn_snake(
    commands: &mut Commands,
    layout: &ArenaLayout,
    player: PlayerId,
    start_pos: IVec2,
    direction: Direction,
) -> Entity {
    let head_color = match player {
        PlayerId::Player1 => COLOR_P1_HEAD,
        PlayerId::Player2 => COLOR_P2_HEAD,
    };
    let body_color = match player {
        PlayerId::Player1 => COLOR_P1,
        PlayerId::Player2 => COLOR_P2,
    };
    let eye_color = match player {
        PlayerId::Player1 => COLOR_P1_EYES,
        PlayerId::Player2 => COLOR_P2_EYES,
    };

    let head_world = layout.grid_to_world(start_pos);
    let head_size = (layout.cell_size * 1.6).max(3.0);
    let seg_size = (layout.cell_size * 1.0).max(1.5);

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
                color: body_color,
                custom_size: Some(Vec2::splat(seg_size)),
                ..default()
            },
            Transform::from_translation(seg_world.extend(1.0)),
        )).id();

        segment_entities.push_back(seg_entity);
    }

    // Spawn Head entity with eye children for directional visual feedback
    let head_entity = commands.spawn((
        SnakeHead { player },
        Sprite {
            color: head_color,
            custom_size: Some(Vec2::splat(head_size)),
            ..default()
        },
        Transform {
            translation: head_world.extend(2.0),
            rotation: Quat::from_rotation_z(direction.angle()),
            ..default()
        },
    )).with_children(|head| {
        // Left Eye
        head.spawn((
            SnakeEye { player, is_left: true },
            Sprite {
                color: eye_color,
                custom_size: Some(Vec2::splat((head_size * 0.3).max(1.5))),
                ..default()
            },
            Transform::from_xyz(head_size * 0.25, head_size * 0.25, 0.1),
        ));

        // Right Eye
        head.spawn((
            SnakeEye { player, is_left: false },
            Sprite {
                color: eye_color,
                custom_size: Some(Vec2::splat((head_size * 0.3).max(1.5))),
                ..default()
            },
            Transform::from_xyz(head_size * 0.25, -head_size * 0.25, 0.1),
        ));
    }).id();

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
        head_entity,
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
        if let Ok(mut transform) = head_transforms.get_mut(snake.head_entity) {
            transform.rotation = Quat::from_rotation_z(snake.next_direction.angle());
        }
    }
}

/// Respawn a snake in an empty, safe location on the grid
pub fn respawn_snake<F: bevy::ecs::query::QueryFilter>(
    commands: &mut Commands,
    layout: &ArenaLayout,
    snake: &mut SnakeState,
    other_snakes_bodies: &[&VecDeque<IVec2>],
    head_transforms: &mut Query<&mut Transform, F>,
) {
    snake.respawns += 1;

    // Despawn all existing body segments
    for entity in snake.segment_entities.drain(..) {
        commands.entity(entity).despawn();
    }
    snake.body.clear();
    snake.growth_pending = 0;

    let mut rng = rand::rng();

    // Find a suitable empty location with space to move
    let mut best_pos = IVec2::new(
        rng.random_range(20..(GRID_WIDTH - 20)),
        rng.random_range(20..(GRID_HEIGHT - 20)),
    );
    let mut best_dir = Direction::Right;

    for _ in 0..60 {
        let candidate_x = rng.random_range(16..(GRID_WIDTH - 16));
        let candidate_y = rng.random_range(16..(GRID_HEIGHT - 16));
        let candidate = IVec2::new(candidate_x, candidate_y);

        let dirs = [Direction::Right, Direction::Left, Direction::Up, Direction::Down];
        let dir = dirs[rng.random_range(0..4)];

        // Check clearance for candidate and 6 cells in direction
        let mut clear = true;
        for step in 0..INITIAL_SNAKE_LENGTH as i32 {
            let check_cell = candidate + dir.to_ivec2() * step;
            if check_cell.x < 5 || check_cell.x >= GRID_WIDTH - 5 || check_cell.y < 5 || check_cell.y >= GRID_HEIGHT - 5 {
                clear = false;
                break;
            }

            for other_body in other_snakes_bodies {
                if other_body.contains(&check_cell) {
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

    let body_color = match snake.player {
        PlayerId::Player1 => COLOR_P1,
        PlayerId::Player2 => COLOR_P2,
    };
    let seg_size = (layout.cell_size * 1.0).max(1.5);

    // Spawn new body segments trailing behind respawn position
    let step_back = best_dir.opposite().to_ivec2();
    for i in 1..INITIAL_SNAKE_LENGTH {
        let seg_cell = best_pos + step_back * (i as i32);
        snake.body.push_back(seg_cell);

        let seg_world = layout.grid_to_world(seg_cell);
        let seg_entity = commands.spawn((
            SnakeSegment { player: snake.player, index: i },
            Sprite {
                color: body_color,
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
