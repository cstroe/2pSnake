use bevy::prelude::*;
use std::collections::VecDeque;
use std::f32::consts::{FRAC_PI_2, PI};
use rand::RngExt;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;
use crate::svg_utils::{SpriteAssets, SnakeSegmentKind};

/// Computes the world-space sprite size for any snake segment or head cell so that adjacent
/// grid cells tile seamlessly without gaps.
pub fn snake_cell_sprite_size(cell_size: f32) -> f32 {
    cell_size.max(3.0)
}

/// Converts a grid delta between adjacent snake cells into a cardinal unit offset `(-1..=1, -1..=1)`.
fn normalize_step(delta: IVec2) -> (i32, i32) {
    (delta.x.signum(), delta.y.signum())
}

/// Given a snake's `body` queue (`body[0]` = head, `body[len-1]` = tail) and the segment's
/// index `body_idx` (`1..body.len()`), returns the appropriate [`SnakeSegmentKind`] and
/// counter-clockwise Z rotation angle (in radians) for that segment.
pub fn compute_segment_kind_and_angle(body: &VecDeque<IVec2>, body_idx: usize) -> (SnakeSegmentKind, f32) {
    let len = body.len();
    if body_idx == 0 || body_idx >= len {
        return (SnakeSegmentKind::Straight, 0.0);
    }

    let curr = body[body_idx];
    let prev = body[body_idx - 1]; // Closer to head
    let d_prev = normalize_step(prev - curr);

    // Tail segment (last element of body)
    if body_idx == len - 1 {
        // Default tail template connects at its RIGHT edge (+X) and tapers to LEFT (-X).
        // Rotate so the open connection edge faces `prev` (`d_prev`).
        let angle = match d_prev {
            (1, 0) => 0.0,          // Connects Right (+X)
            (0, 1) => FRAC_PI_2,    // Connects Up (+Y)
            (-1, 0) => PI,          // Connects Left (-X)
            (0, -1) => -FRAC_PI_2,  // Connects Down (-Y)
            _ => 0.0,
        };
        return (SnakeSegmentKind::Tail, angle);
    }

    let next = body[body_idx + 1]; // Closer to tail
    let d_next = normalize_step(next - curr);

    // Straight segment (collinear neighbors)
    if d_prev.0 == -d_next.0 && d_prev.1 == -d_next.1 {
        // Default straight template is Horizontal (Left <-> Right).
        let angle = if d_prev.0 != 0 {
            0.0
        } else {
            FRAC_PI_2
        };
        return (SnakeSegmentKind::Straight, angle);
    }

    // Corner / turn segment (orthogonal neighbors).
    // Default turn template connects RIGHT (+X: (1, 0)) and BOTTOM (-Y: (0, -1) in Bevy world space).
    let has_right = d_prev == (1, 0) || d_next == (1, 0);
    let has_left = d_prev == (-1, 0) || d_next == (-1, 0);
    let has_up = d_prev == (0, 1) || d_next == (0, 1);
    let has_down = d_prev == (0, -1) || d_next == (0, -1);

    let angle = if has_right && has_down {
        0.0
    } else if has_right && has_up {
        FRAC_PI_2
    } else if has_left && has_up {
        PI
    } else if has_left && has_down {
        -FRAC_PI_2
    } else {
        0.0
    };

    (SnakeSegmentKind::Turn, angle)
}

/// Computes the head's orientation `(is_turned, angle, flip_y)` given the snake's `body`
/// (`body[0]` = head cell, `body[1]` = first neck/body segment cell) and `next_direction`.
///
/// - When the head faces directly away from `body[1]` (straight ahead), `is_turned = false`,
///   `angle = next_direction.angle()`, and `flip_y = false`.
/// - When the player turns the head 90 degrees before the snake steps forward, `is_turned = true`:
///   the intermediate curved-neck head SVG (`snake_head_turn_template.svg`) is used, rotated by
///   `next_direction.angle()`, and `flip_y` is set when the trailing body segment is to the head's
///   left-hand side (left turn) so the curved neck connects seamlessly to `body[1]`.
pub fn compute_head_pose(body: &VecDeque<IVec2>, next_direction: Direction) -> (bool, f32, bool) {
    let angle = next_direction.angle();
    if body.len() < 2 {
        return (false, angle, false);
    }

    let d_neck = normalize_step(body[1] - body[0]);
    let f = next_direction.to_ivec2();
    let f_step = (f.x.signum(), f.y.signum());

    // Straight ahead: trailing segment is directly behind next_direction
    if d_neck == (-f_step.0, -f_step.1) || d_neck == f_step {
        return (false, angle, false);
    }

    // 2D cross product `f.x * d_neck.y - f.y * d_neck.x`:
    // - If `< 0`, `d_neck` is 90 deg clockwise from `f` (on the head's right-hand side -> unflipped turn template).
    // - If `> 0`, `d_neck` is 90 deg counter-clockwise from `f` (on the head's left-hand side -> flip_y = true).
    let cross = f_step.0 * d_neck.1 - f_step.1 * d_neck.0;
    let flip_y = cross > 0;

    (true, angle, flip_y)
}

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
    let head_image = sprites.snake_head(player, false);

    let head_world = layout.grid_to_world(start_pos);
    let tile_size = snake_cell_sprite_size(layout.cell_size);

    // Build initial body cell positions trailing behind start_pos
    let mut body = VecDeque::new();
    let mut segment_entities = VecDeque::new();

    body.push_back(start_pos);

    let step_back = direction.opposite().to_ivec2();
    for i in 1..INITIAL_SNAKE_LENGTH {
        let seg_cell = start_pos + step_back * (i as i32);
        body.push_back(seg_cell);
    }

    for i in 1..INITIAL_SNAKE_LENGTH {
        let seg_cell = body[i];
        let (kind, angle) = compute_segment_kind_and_angle(&body, i);
        let seg_image = sprites.snake_segment(player, kind, false);
        let seg_world = layout.grid_to_world(seg_cell);
        let seg_entity = commands.spawn((
            SnakeSegment { player, index: i },
            Sprite {
                image: seg_image,
                custom_size: Some(Vec2::splat(tile_size)),
                ..default()
            },
            Transform {
                translation: seg_world.extend(1.0),
                rotation: Quat::from_rotation_z(angle),
                ..default()
            },
        )).id();

        segment_entities.push_back(seg_entity);
    }

    // Spawn Head entity using vector SVG texture
    // Designed pointing Right (0 deg), so rotation by direction.angle() naturally faces user's direction
    let head_entity = commands.spawn((
        SnakeHead { player },
        Sprite {
            image: head_image,
            custom_size: Some(Vec2::splat(tile_size)),
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

/// Updates head rotation and intermediate turned-neck texture IMMEDIATELY whenever the player
/// changes direction, fulfilling the requirement:
/// "As soon as the player presses a direction, the head of the snake should be
///  looking in the direction that the user pressed, so that the user can see
///  where the snake will be moving."
/// While turned prior to stepping, swaps to the curved-neck head SVG so the head remains
/// seamlessly connected to the trailing body segment.
pub fn update_snake_head_visuals(
    sprites: Res<SpriteAssets>,
    snakes: Query<&SnakeState>,
    mut head_query: Query<(&mut Transform, Option<&mut Sprite>), With<SnakeHead>>,
) {
    for snake in &snakes {
        if snake.death_timer > 0.0 {
            continue;
        }
        let (is_turned, angle, flip_y) = compute_head_pose(&snake.body, snake.next_direction);
        if let Ok((mut transform, sprite_opt)) = head_query.get_mut(snake.head_entity) {
            transform.rotation = Quat::from_rotation_z(angle);
            if let Some(mut sprite) = sprite_opt {
                sprite.image = sprites.snake_head_variant(snake.player, is_turned, false);
                sprite.flip_y = flip_y;
            }
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

    let tile_size = snake_cell_sprite_size(layout.cell_size);

    // Populate body cells trailing behind respawn position
    let step_back = best_dir.opposite().to_ivec2();
    for i in 1..INITIAL_SNAKE_LENGTH {
        let seg_cell = best_pos + step_back * (i as i32);
        snake.body.push_back(seg_cell);
    }

    for i in 1..INITIAL_SNAKE_LENGTH {
        let seg_cell = snake.body[i];
        let (kind, angle) = compute_segment_kind_and_angle(&snake.body, i);
        let seg_image = sprites.snake_segment(snake.player, kind, false);
        let seg_world = layout.grid_to_world(seg_cell);
        let seg_entity = commands.spawn((
            SnakeSegment { player: snake.player, index: i },
            Sprite {
                image: seg_image,
                custom_size: Some(Vec2::splat(tile_size)),
                ..default()
            },
            Transform {
                translation: seg_world.extend(1.0),
                rotation: Quat::from_rotation_z(angle),
                ..default()
            },
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

/// Visual effects and segment orientation for snakes:
/// - Updates each segment's texture (Straight, Turn, Tail) and rotation angle to match the snake's current path.
/// - Death transition: 1.0 second total duration (stops moving, turns grey, fades out to 0 alpha, then respawns).
/// - Pepper speed boost: Rapid flashing strobe effect (approx 10 Hz) while active.
/// - Normal: Resets head and body segments to normal color textures and Color::WHITE.
pub fn update_snake_boost_flashing(
    time: Res<Time>,
    sprites: Res<SpriteAssets>,
    snakes: Query<&SnakeState>,
    mut head_sprites: Query<&mut Sprite, (With<SnakeHead>, Without<SnakeSegment>)>,
    mut segment_query: Query<(&mut Sprite, &mut Transform), (With<SnakeSegment>, Without<SnakeHead>)>,
) {
    let t = time.elapsed_secs();
    // Fast pulsating strobe effect (approx 10 Hz)
    let flash_phase = (t * 22.0).sin();

    for snake in &snakes {
        let (is_gray, tint) = if snake.death_timer > 0.0 {
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

            let tint = if is_gray {
                Color::srgba(0.85, 0.85, 0.85, alpha)
            } else {
                Color::srgba(1.0, 1.0, 1.0, alpha)
            };
            (is_gray, tint)
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
            (false, tint)
        };

        let (is_turned, head_angle, head_flip_y) = compute_head_pose(&snake.body, snake.next_direction);
        let head_img = sprites.snake_head_variant(snake.player, is_turned, is_gray);
        if let Ok(mut sprite) = head_sprites.get_mut(snake.head_entity) {
            sprite.image = head_img;
            sprite.color = tint;
            sprite.flip_y = head_flip_y;
        }
        let _ = head_angle;

        for (i, &seg_entity) in snake.segment_entities.iter().enumerate() {
            let body_idx = i + 1;
            let (kind, angle) = compute_segment_kind_and_angle(&snake.body, body_idx);
            let seg_img = sprites.snake_segment(snake.player, kind, is_gray);

            if let Ok((mut sprite, mut transform)) = segment_query.get_mut(seg_entity) {
                sprite.image = seg_img;
                sprite.color = tint;
                transform.rotation = Quat::from_rotation_z(angle);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_head_pose_straight_and_turns() {
        // Snake moving Right: head at (10, 5), neck at (9, 5)
        let body = VecDeque::from(vec![
            IVec2::new(10, 5),
            IVec2::new(9, 5),
            IVec2::new(8, 5),
        ]);

        // Continuing straight Right -> not turned, angle 0, flip_y false
        let (turned, angle, flip_y) = compute_head_pose(&body, Direction::Right);
        assert!(!turned);
        assert!((angle - 0.0).abs() < 1e-5);
        assert!(!flip_y);

        // Turning Up from Right: head faces Up (+Y), neck is to the Left (-X), which is the head's left-hand side -> turned, angle +PI/2, flip_y = true
        let (turned_up, angle_up, flip_up) = compute_head_pose(&body, Direction::Up);
        assert!(turned_up);
        assert!((angle_up - FRAC_PI_2).abs() < 1e-5);
        assert!(flip_up);

        // Turning Down from Right: head faces Down (-Y), neck is to the Left (-X), which is the head's right-hand side -> turned, angle -PI/2, flip_y = false
        let (turned_down, angle_down, flip_down) = compute_head_pose(&body, Direction::Down);
        assert!(turned_down);
        assert!((angle_down - (-FRAC_PI_2)).abs() < 1e-5);
        assert!(!flip_down);
    }

    #[test]
    fn test_compute_segment_kind_and_angle_straight_and_tail() {
        // Horizontal snake facing Right: head at (10, 5), body at (9, 5), (8, 5), tail at (7, 5)
        let body = VecDeque::from(vec![
            IVec2::new(10, 5),
            IVec2::new(9, 5),
            IVec2::new(8, 5),
            IVec2::new(7, 5),
        ]);

        let (k1, a1) = compute_segment_kind_and_angle(&body, 1);
        assert_eq!(k1, SnakeSegmentKind::Straight);
        assert!((a1 - 0.0).abs() < 1e-5);

        let (k2, a2) = compute_segment_kind_and_angle(&body, 2);
        assert_eq!(k2, SnakeSegmentKind::Straight);
        assert!((a2 - 0.0).abs() < 1e-5);

        // Tail at (7, 5) connects to (8, 5) on its Right (+X) -> angle 0.0
        let (k3, a3) = compute_segment_kind_and_angle(&body, 3);
        assert_eq!(k3, SnakeSegmentKind::Tail);
        assert!((a3 - 0.0).abs() < 1e-5);

        // Vertical snake facing Up: head at (5, 10), body at (5, 9), tail at (5, 8)
        let v_body = VecDeque::from(vec![
            IVec2::new(5, 10),
            IVec2::new(5, 9),
            IVec2::new(5, 8),
        ]);
        let (vk1, va1) = compute_segment_kind_and_angle(&v_body, 1);
        assert_eq!(vk1, SnakeSegmentKind::Straight);
        assert!((va1 - FRAC_PI_2).abs() < 1e-5);

        // Tail at (5, 8) connects to (5, 9) Up (+Y) -> angle +PI/2
        let (vk2, va2) = compute_segment_kind_and_angle(&v_body, 2);
        assert_eq!(vk2, SnakeSegmentKind::Tail);
        assert!((va2 - FRAC_PI_2).abs() < 1e-5);
    }

    #[test]
    fn test_compute_segment_kind_and_angle_all_four_turns() {
        // 1. Turn connecting Right (+X) and Down (-Y) -> 0.0 rad
        let turn_right_down = VecDeque::from(vec![
            IVec2::new(6, 5), // Right of (5,5)
            IVec2::new(5, 5), // Corner
            IVec2::new(5, 4), // Below (5,5)
        ]);
        let (k, a) = compute_segment_kind_and_angle(&turn_right_down, 1);
        assert_eq!(k, SnakeSegmentKind::Turn);
        assert!((a - 0.0).abs() < 1e-5);

        // 2. Turn connecting Right (+X) and Up (+Y) -> +PI/2 rad
        let turn_right_up = VecDeque::from(vec![
            IVec2::new(6, 5), // Right of (5,5)
            IVec2::new(5, 5), // Corner
            IVec2::new(5, 6), // Above (5,5)
        ]);
        let (k, a) = compute_segment_kind_and_angle(&turn_right_up, 1);
        assert_eq!(k, SnakeSegmentKind::Turn);
        assert!((a - FRAC_PI_2).abs() < 1e-5);

        // 3. Turn connecting Left (-X) and Up (+Y) -> PI rad
        let turn_left_up = VecDeque::from(vec![
            IVec2::new(4, 5), // Left of (5,5)
            IVec2::new(5, 5), // Corner
            IVec2::new(5, 6), // Above (5,5)
        ]);
        let (k, a) = compute_segment_kind_and_angle(&turn_left_up, 1);
        assert_eq!(k, SnakeSegmentKind::Turn);
        assert!((a - PI).abs() < 1e-5);

        // 4. Turn connecting Left (-X) and Down (-Y) -> -PI/2 rad
        let turn_left_down = VecDeque::from(vec![
            IVec2::new(4, 5), // Left of (5,5)
            IVec2::new(5, 5), // Corner
            IVec2::new(5, 4), // Below (5,5)
        ]);
        let (k, a) = compute_segment_kind_and_angle(&turn_left_down, 1);
        assert_eq!(k, SnakeSegmentKind::Turn);
        assert!((a - (-FRAC_PI_2)).abs() < 1e-5);
    }

    #[test]
    fn test_template_svgs_render_for_both_players() {
        use crate::svg_utils::{colorize_snake_svg, render_svg_to_image};
        let templates = [
            include_str!("../assets/svg/snake_head_template.svg"),
            include_str!("../assets/svg/snake_head_turn_template.svg"),
            include_str!("../assets/svg/snake_body_straight_template.svg"),
            include_str!("../assets/svg/snake_body_turn_template.svg"),
            include_str!("../assets/svg/snake_tail_template.svg"),
        ];
        for player in [PlayerId::Player1, PlayerId::Player2] {
            for tmpl in templates {
                let colored = colorize_snake_svg(tmpl, player);
                let img = render_svg_to_image(&colored, 64, 64);
                assert!(img.is_ok(), "Failed to render template SVG for {:?}: {:?}", player, img.err());
            }
        }
    }
}


