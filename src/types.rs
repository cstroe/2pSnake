use bevy::prelude::*;
use rand::RngExt;

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    StartScreen,
    Settings,
    Credits,
    ControllerSelect,
    Countdown,
    Playing,
    RoundOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerId {
    Player1,
    Player2,
}

#[allow(dead_code)]
impl PlayerId {
    pub fn name(self) -> &'static str {
        match self {
            PlayerId::Player1 => "Player 1 (Cyan)",
            PlayerId::Player2 => "Player 2 (Orange)",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_ivec2(self) -> IVec2 {
        match self {
            Direction::Up => IVec2::new(0, 1),
            Direction::Down => IVec2::new(0, -1),
            Direction::Left => IVec2::new(-1, 0),
            Direction::Right => IVec2::new(1, 0),
        }
    }

    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn angle(self) -> f32 {
        match self {
            Direction::Right => 0.0,
            Direction::Up => std::f32::consts::FRAC_PI_2,
            Direction::Left => std::f32::consts::PI,
            Direction::Down => -std::f32::consts::FRAC_PI_2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FruitType {
    Apple,
    Strawberry,
    Orange,
    Grapes,
    Banana,
    Watermelon,
    Starfruit,
    HotPepper,
}

impl FruitType {
    pub fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..8) {
            0 => FruitType::Apple,
            1 => FruitType::Strawberry,
            2 => FruitType::Orange,
            3 => FruitType::Grapes,
            4 => FruitType::Banana,
            5 => FruitType::Watermelon,
            6 => FruitType::Starfruit,
            _ => FruitType::HotPepper,
        }
    }

    pub fn points(self) -> u32 {
        match self {
            FruitType::Apple => 1,
            FruitType::Strawberry => 1,
            FruitType::Orange => 2,
            FruitType::Grapes => 2,
            FruitType::Banana => 2,
            FruitType::Watermelon => 3,
            FruitType::Starfruit => 4,
            FruitType::HotPepper => 3,
        }
    }

    pub fn growth(self) -> u32 {
        match self {
            FruitType::Apple => 2,
            FruitType::Strawberry => 3,
            FruitType::Orange => 3,
            FruitType::Grapes => 4,
            FruitType::Banana => 4,
            FruitType::Watermelon => 5,
            FruitType::Starfruit => 6,
            FruitType::HotPepper => 2,
        }
    }

    pub fn color(self) -> Color {
        match self {
            FruitType::Apple => Color::srgb(1.0, 0.15, 0.2),       // Vivid Red
            FruitType::Strawberry => Color::srgb(1.0, 0.3, 0.5),  // Neon Pink
            FruitType::Orange => Color::srgb(1.0, 0.6, 0.0),      // Neon Orange
            FruitType::Grapes => Color::srgb(0.75, 0.2, 1.0),     // Neon Purple
            FruitType::Banana => Color::srgb(1.0, 0.9, 0.1),      // Neon Yellow
            FruitType::Watermelon => Color::srgb(0.1, 0.9, 0.4),  // Neon Green
            FruitType::Starfruit => Color::srgb(1.0, 1.0, 0.4),   // Radiant Gold
            FruitType::HotPepper => Color::srgb(1.0, 0.25, 0.0),  // Fiery Pepper Red
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            FruitType::Apple => "Apple",
            FruitType::Strawberry => "Strawberry",
            FruitType::Orange => "Orange",
            FruitType::Grapes => "Grapes",
            FruitType::Banana => "Banana",
            FruitType::Watermelon => "Watermelon",
            FruitType::Starfruit => "Starfruit",
            FruitType::HotPepper => "Hot Pepper",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControllerDevice {
    Gamepad(Entity),
    KeyboardP1,
    KeyboardP2,
}

impl ControllerDevice {
    pub fn display_name(&self, gamepad_name: Option<&str>) -> String {
        match self {
            ControllerDevice::Gamepad(entity) => {
                if let Some(name) = gamepad_name {
                    format!("{name} (ID: {})", entity.index())
                } else {
                    format!("Gamepad #{}", entity.index())
                }
            }
            ControllerDevice::KeyboardP1 => "Keyboard (WASD + Space)".to_string(),
            ControllerDevice::KeyboardP2 => "Keyboard (Arrows + Enter)".to_string(),
        }
    }
}
