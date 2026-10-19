use bevy::prelude::*;
use crate::constants::*;
use crate::resources::*;
use crate::components::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup_playfield(
    mut commands: Commands,
    windows: Query<&Window>,
    mut layout: ResMut<ArenaLayout>,
) {
    if let Some(window) = windows.iter().next() {
        layout.update(window.width(), window.height());
    }

    let arena_size = Vec2::new(
        GRID_WIDTH as f32 * layout.cell_size,
        GRID_HEIGHT as f32 * layout.cell_size,
    );

    commands.spawn((
        PlayfieldBg,
        Sprite {
            color: COLOR_ARENA_BG,
            custom_size: Some(arena_size),
            ..default()
        },
        Transform::from_translation(layout.grid_center.extend(-10.0)),
    ));
}

pub fn update_arena_layout(
    windows: Query<&Window>,
    mut layout: ResMut<ArenaLayout>,
    mut bg_query: Query<(&mut Sprite, &mut Transform), With<PlayfieldBg>>,
    mut head_query: Query<(&mut Transform, &mut Sprite), (With<SnakeHead>, Without<PlayfieldBg>, Without<Fruit>, Without<SnakeSegment>)>,
    mut segment_query: Query<(&mut Transform, &mut Sprite), (With<SnakeSegment>, Without<PlayfieldBg>, Without<Fruit>, Without<SnakeHead>)>,
    mut fruit_query: Query<(&mut Transform, &mut Sprite, &Fruit), (Without<PlayfieldBg>, Without<SnakeHead>, Without<SnakeSegment>)>,
    snakes_query: Query<&SnakeState>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let cur_w = window.width();
    let cur_h = window.height();

    // Check if window resized or if layout has not been computed yet
    let size_changed = (cur_w - layout.window_size.x).abs() > 0.5 
                    || (cur_h - layout.window_size.y).abs() > 0.5
                    || layout.cell_size <= 0.01;

    if size_changed {
        layout.update(cur_w, cur_h);

        // Update playfield background
        let arena_size = Vec2::new(
            GRID_WIDTH as f32 * layout.cell_size,
            GRID_HEIGHT as f32 * layout.cell_size,
        );

        for (mut sprite, mut transform) in &mut bg_query {
            sprite.custom_size = Some(arena_size);
            transform.translation = layout.grid_center.extend(-10.0);
        }

        // Adjust fruit size and reposition to match new grid scaling
        let fruit_size = (layout.cell_size * 2.2).max(4.0);
        for (mut transform, mut sprite, fruit) in &mut fruit_query {
            sprite.custom_size = Some(Vec2::splat(fruit_size));
            transform.translation = layout.grid_to_world(fruit.cell).extend(1.5);
        }

        // Adjust snake head & segments size and reposition to match new grid scaling
        let head_size = (layout.cell_size * 2.2).max(4.0);
        let seg_size = (layout.cell_size * 1.5).max(3.0);

        for snake in &snakes_query {
            if let Ok((mut transform, mut sprite)) = head_query.get_mut(snake.head_entity) {
                sprite.custom_size = Some(Vec2::splat(head_size));
                if let Some(&head_cell) = snake.body.front() {
                    transform.translation = layout.grid_to_world(head_cell).extend(2.0);
                }
            }

            for (i, &entity) in snake.segment_entities.iter().enumerate() {
                if let Some(&seg_cell) = snake.body.get(i + 1) {
                    if let Ok((mut transform, mut sprite)) = segment_query.get_mut(entity) {
                        sprite.custom_size = Some(Vec2::splat(seg_size));
                        transform.translation = layout.grid_to_world(seg_cell).extend(1.0);
                    }
                }
            }
        }
    }
}

pub fn draw_arena_borders(
    layout: Res<ArenaLayout>,
    mut gizmos: Gizmos,
) {
    let arena_w = GRID_WIDTH as f32 * layout.cell_size;
    let arena_h = GRID_HEIGHT as f32 * layout.cell_size;
    let size = Vec2::new(arena_w, arena_h);

    // Outer neon glow rectangle
    gizmos.rect_2d(layout.grid_center, size, COLOR_ARENA_BORDER);

    // Subtle inner accent border
    let inner_size = size - Vec2::splat(4.0);
    if inner_size.x > 0.0 && inner_size.y > 0.0 {
        gizmos.rect_2d(layout.grid_center, inner_size, Color::srgba(0.0, 0.7, 0.9, 0.25));
    }

    // Corner accents
    let half_w = arena_w / 2.0;
    let half_h = arena_h / 2.0;
    let corner_len = (16.0 * layout.cell_size).clamp(8.0, 32.0);

    let corners = [
        Vec2::new(-half_w, -half_h) + layout.grid_center,
        Vec2::new(half_w, -half_h) + layout.grid_center,
        Vec2::new(half_w, half_h) + layout.grid_center,
        Vec2::new(-half_w, half_h) + layout.grid_center,
    ];

    // Draw stylish corner brackets
    let p0 = corners[0];
    gizmos.line_2d(p0, p0 + Vec2::new(corner_len, 0.0), COLOR_P1);
    gizmos.line_2d(p0, p0 + Vec2::new(0.0, corner_len), COLOR_P1);

    let p1 = corners[1];
    gizmos.line_2d(p1, p1 + Vec2::new(-corner_len, 0.0), COLOR_P2);
    gizmos.line_2d(p1, p1 + Vec2::new(0.0, corner_len), COLOR_P2);

    let p2 = corners[2];
    gizmos.line_2d(p2, p2 + Vec2::new(-corner_len, 0.0), COLOR_P2);
    gizmos.line_2d(p2, p2 + Vec2::new(0.0, -corner_len), COLOR_P2);

    let p3 = corners[3];
    gizmos.line_2d(p3, p3 + Vec2::new(corner_len, 0.0), COLOR_P1);
    gizmos.line_2d(p3, p3 + Vec2::new(0.0, -corner_len), COLOR_P1);
}
