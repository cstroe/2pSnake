use bevy::prelude::*;
use std::collections::VecDeque;
use crate::types::*;

#[allow(dead_code)]
#[derive(Component)]
pub struct SnakeHead {
    pub player: PlayerId,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct SnakeSegment {
    pub player: PlayerId,
    pub index: usize,
}

#[allow(dead_code)]
#[derive(Component)]
pub struct SnakeEye {
    pub player: PlayerId,
    pub is_left: bool,
}

#[derive(Component)]
pub struct SnakeState {
    pub player: PlayerId,
    pub direction: Direction,
    pub next_direction: Direction,
    pub body: VecDeque<IVec2>, // body[0] is head cell, body[1..] are segment cells
    pub segment_entities: VecDeque<Entity>,
    pub growth_pending: u32,
    pub score: u32,
    pub respawns: u32,
    pub head_entity: Entity,
    pub stored_peppers: u32,
    pub boost_timer: f32,
    pub move_timer: f32,
}

#[derive(Component)]
pub struct Fruit {
    pub fruit_type: FruitType,
    pub cell: IVec2,
    pub phase_offset: f32,
}

#[derive(Component)]
pub struct PlayfieldBg;

// UI Marker components
#[derive(Component)]
pub struct ControllerSelectRoot;

#[derive(Component)]
pub struct P1SlotStatusText;

#[derive(Component)]
pub struct P2SlotStatusText;

#[derive(Component)]
pub struct P1DeviceText;

#[derive(Component)]
pub struct P2DeviceText;

#[derive(Component)]
pub struct AvailableDevicesText;

#[derive(Component)]
pub struct BottomStatusText;

#[derive(Component)]
pub struct CountdownRoot;

#[derive(Component)]
pub struct CountdownText;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct HudP1ScoreText;

#[derive(Component)]
pub struct HudP2ScoreText;

#[derive(Component)]
pub struct HudTimeText;

#[derive(Component)]
pub struct HudSpeedText;

#[derive(Component)]
pub struct HudMessageText;

#[derive(Component)]
pub struct RoundOverRoot;

#[derive(Component)]
pub struct RoundOverWinnerText;

#[derive(Component)]
pub struct RoundOverDetailsText;

#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Component)]
pub struct HudPlayerPepperSlot {
    pub player: PlayerId,
    pub slot_index: usize,
}

#[derive(Component)]
pub struct HudPlayerBoostText {
    pub player: PlayerId,
}
