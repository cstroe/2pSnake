use bevy::prelude::*;
use rand::RngExt;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;

pub fn spawn_initial_fruits(
    mut commands: Commands,
    layout: Res<ArenaLayout>,
    snakes: Query<&SnakeState>,
) {
    let mut rng = rand::rng();
    let mut occupied = Vec::new();

    for snake in &snakes {
        for cell in &snake.body {
            occupied.push(*cell);
        }
    }

    for i in 0..NUM_CONCURRENT_FRUITS {
        spawn_one_fruit(&mut commands, &layout, &mut occupied, i as f32 * 0.3, &mut rng);
    }
}

pub fn spawn_one_fruit(
    commands: &mut Commands,
    layout: &ArenaLayout,
    occupied: &mut Vec<IVec2>,
    phase_offset: f32,
    rng: &mut impl rand::Rng,
) -> Option<Entity> {
    for _ in 0..100 {
        let x = rng.random_range(10..(GRID_WIDTH - 10));
        let y = rng.random_range(10..(GRID_HEIGHT - 10));
        let cell = IVec2::new(x, y);

        if !occupied.contains(&cell) {
            occupied.push(cell);

            let fruit_type = FruitType::random();
            let world_pos = layout.grid_to_world(cell);
            let fruit_size = (layout.cell_size * 2.2).max(4.0);

            let entity = commands.spawn((
                Fruit {
                    fruit_type,
                    cell,
                    phase_offset,
                },
                Sprite {
                    color: fruit_type.color(),
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

/// Gentle pulsating glow effect for fruits so players can spot them instantly
pub fn animate_fruits(
    time: Res<Time>,
    layout: Res<ArenaLayout>,
    mut query: Query<(&mut Sprite, &Fruit)>,
) {
    let base_size = (layout.cell_size * 2.2).max(4.0);
    let t = time.elapsed_secs();

    for (mut sprite, fruit) in &mut query {
        let pulse = 1.0 + 0.25 * (t * 4.0 + fruit.phase_offset).sin();
        sprite.custom_size = Some(Vec2::splat(base_size * pulse));
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
