use bevy::prelude::*;
use rand::RngExt;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;
use crate::svg_utils::SpriteAssets;

pub fn spawn_initial_fruits(
    mut commands: Commands,
    layout: Res<ArenaLayout>,
    sprites: Res<SpriteAssets>,
    snakes: Query<&SnakeState>,
) {
    let mut rng = rand::rng();
    let mut occupied = Vec::new();

    for snake in &snakes {
        for cell in &snake.body {
            occupied.push(*cell);
        }
    }

    for _ in 0..NUM_CONCURRENT_FRUITS {
        spawn_one_fruit(&mut commands, &layout, &sprites, &mut occupied, &mut rng);
    }
}

pub fn fruit_sprite_size(cell_size: f32) -> f32 {
    (cell_size * 1.54).max(3.0)
}

pub fn spawn_one_fruit(
    commands: &mut Commands,
    layout: &ArenaLayout,
    sprites: &SpriteAssets,
    occupied: &mut Vec<IVec2>,
    rng: &mut impl rand::Rng,
) -> Option<Entity> {
    for _ in 0..100 {
        let x = rng.random_range(5..(GRID_WIDTH - 5));
        let y = rng.random_range(5..(GRID_HEIGHT - 5));
        let cell = IVec2::new(x, y);

        if !occupied.contains(&cell) {
            occupied.push(cell);

            let fruit_type = FruitType::random();
            let world_pos = layout.grid_to_world(cell);
            let fruit_size = fruit_sprite_size(layout.cell_size);

            let entity = commands.spawn((
                Fruit {
                    fruit_type,
                    cell,
                },
                Sprite {
                    image: sprites.fruit_handle(fruit_type),
                    custom_size: Some(Vec2::splat(fruit_size)),
                    ..default()
                },
                Transform::from_translation(world_pos.extend(1.5)),
            )).id();

            return Some(entity);
        }
    }
    None
}

/// Keeps fruits at their static base size without pulsating larger and smaller
pub fn animate_fruits(
    layout: Res<ArenaLayout>,
    mut query: Query<&mut Sprite, With<Fruit>>,
) {
    let base_size = fruit_sprite_size(layout.cell_size);

    for mut sprite in &mut query {
        sprite.custom_size = Some(Vec2::splat(base_size));
    }
}

pub fn cleanup_fruits(
    mut commands: Commands,
    query: Query<Entity, With<Fruit>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
