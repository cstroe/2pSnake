use bevy::prelude::*;
use crate::constants::*;
use crate::types::*;
use crate::resources::*;
use crate::components::*;

pub fn setup_countdown_ui(
    mut commands: Commands,
    mut countdown: ResMut<CountdownState>,
) {
    countdown.timer = Timer::from_seconds(1.0, TimerMode::Once);
    countdown.step = CountdownStep::Three;

    commands.spawn((
        CountdownRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            CountdownText,
            Text::new("3"),
            TextFont {
                font_size: FontSize::Px(120.0),
                ..default()
            },
            TextColor(Color::srgb(0.0, 0.9, 1.0)),
        ));

        parent.spawn((
            Text::new("GET READY!"),
            TextFont {
                font_size: FontSize::Px(28.0),
                ..default()
            },
            TextColor(COLOR_TEXT_MUTED),
        ));
    });
}

pub fn update_countdown(
    time: Res<Time>,
    mut countdown: ResMut<CountdownState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut text_query: Query<(&mut Text, &mut TextColor, &mut TextFont), With<CountdownText>>,
) {
    countdown.timer.tick(time.delta());

    if countdown.timer.just_finished() {
        match countdown.step {
            CountdownStep::Three => {
                countdown.step = CountdownStep::Two;
                countdown.timer = Timer::from_seconds(1.0, TimerMode::Once);
                if let Some((mut text, mut color, mut font)) = text_query.iter_mut().next() {
                    **text = "2".to_string();
                    color.0 = Color::srgb(0.3, 0.85, 1.0);
                    font.font_size = FontSize::Px(120.0);
                }
            }
            CountdownStep::Two => {
                countdown.step = CountdownStep::One;
                countdown.timer = Timer::from_seconds(1.0, TimerMode::Once);
                if let Some((mut text, mut color, mut font)) = text_query.iter_mut().next() {
                    **text = "1".to_string();
                    color.0 = Color::srgb(1.0, 0.7, 0.2);
                    font.font_size = FontSize::Px(120.0);
                }
            }
            CountdownStep::One => {
                // "then Start!. Once start dissapears (1 second delay), the game can start"
                countdown.step = CountdownStep::Start;
                countdown.timer = Timer::from_seconds(1.0, TimerMode::Once);
                if let Some((mut text, mut color, mut font)) = text_query.iter_mut().next() {
                    **text = "START!".to_string();
                    color.0 = COLOR_READY_GREEN;
                    font.font_size = FontSize::Px(96.0);
                }
            }
            CountdownStep::Start => {
                countdown.step = CountdownStep::Done;
                // Transition to Playing
                next_state.set(GameState::Playing);
            }
            CountdownStep::Done => {}
        }
    }
}

pub fn cleanup_countdown_ui(
    mut commands: Commands,
    query: Query<Entity, With<CountdownRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
