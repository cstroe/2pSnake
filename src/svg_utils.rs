use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use resvg::usvg::{Options, Tree};
use resvg::tiny_skia::{Pixmap, Transform};
use crate::types::*;

const SVG_SNAKE_HEAD_TEMPLATE: &str = include_str!("../assets/svg/snake_head_template.svg");
const SVG_SNAKE_BODY_STRAIGHT_TEMPLATE: &str = include_str!("../assets/svg/snake_body_straight_template.svg");
const SVG_SNAKE_BODY_TURN_TEMPLATE: &str = include_str!("../assets/svg/snake_body_turn_template.svg");
const SVG_SNAKE_TAIL_TEMPLATE: &str = include_str!("../assets/svg/snake_tail_template.svg");

const SVG_FRUIT_APPLE: &[u8] = include_bytes!("../assets/svg/fruit_apple.svg");
const SVG_FRUIT_STRAWBERRY: &[u8] = include_bytes!("../assets/svg/fruit_strawberry.svg");
const SVG_FRUIT_ORANGE: &[u8] = include_bytes!("../assets/svg/fruit_orange.svg");
const SVG_FRUIT_GRAPES: &[u8] = include_bytes!("../assets/svg/fruit_grapes.svg");
const SVG_FRUIT_BANANA: &[u8] = include_bytes!("../assets/svg/fruit_banana.svg");
const SVG_FRUIT_WATERMELON: &[u8] = include_bytes!("../assets/svg/fruit_watermelon.svg");
const SVG_FRUIT_STARFRUIT: &[u8] = include_bytes!("../assets/svg/fruit_starfruit.svg");
const SVG_FRUIT_PEPPER: &[u8] = include_bytes!("../assets/svg/fruit_pepper.svg");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnakeSegmentKind {
    Straight,
    Turn,
    Tail,
}

/// Applies player-specific color palettes to any of the snake template SVGs.
pub fn colorize_snake_svg(template_svg: &str, player: PlayerId) -> Vec<u8> {
    let colored = match player {
        PlayerId::Player1 => {
            // Player 1: Vivid Cyan / Electric Blue Snake with Golden-Yellow accents
            template_svg
                .replace("#34c759", "#00e5ff") // Highlight
                .replace("#28b44e", "#00b8d4") // Mid-bright
                .replace("#1e9e42", "#0097a7") // Head mid
                .replace("#198236", "#00838f") // Brow mid
                .replace("#146b2c", "#006064") // Cylinder edge shadow
                .replace("#0d5c25", "#004d56") // Deep head shadow
                .replace("#0b4f1f", "#003b46") // Dorsal diamonds & side triangles
                .replace("#083d17", "#00252e") // Outer outline border
                .replace("#f5d126", "#ffd600") // Golden yellow lateral triangles & crest
        }
        PlayerId::Player2 => {
            // Player 2: Fiery Orange / Amber Snake with Bright Yellow accents
            template_svg
                .replace("#34c759", "#ff9100") // Highlight
                .replace("#28b44e", "#ff6d00") // Mid-bright
                .replace("#1e9e42", "#f55500") // Head mid
                .replace("#198236", "#dd4500") // Brow mid
                .replace("#146b2c", "#b23400") // Cylinder edge shadow
                .replace("#0d5c25", "#942900") // Deep head shadow
                .replace("#0b4f1f", "#731c00") // Dorsal diamonds & side triangles
                .replace("#083d17", "#4a1000") // Outer outline border
                .replace("#f5d126", "#ffee58") // Bright yellow lateral triangles & crest
        }
    };
    colored.into_bytes()
}

#[derive(Resource, Default, Clone)]
pub struct SpriteAssets {
    pub p1_head: Handle<Image>,
    pub p1_body: Handle<Image>,
    pub p1_body_turn: Handle<Image>,
    pub p1_tail: Handle<Image>,
    pub p2_head: Handle<Image>,
    pub p2_body: Handle<Image>,
    pub p2_body_turn: Handle<Image>,
    pub p2_tail: Handle<Image>,
    pub p1_head_gray: Handle<Image>,
    pub p1_body_gray: Handle<Image>,
    pub p1_body_turn_gray: Handle<Image>,
    pub p1_tail_gray: Handle<Image>,
    pub p2_head_gray: Handle<Image>,
    pub p2_body_gray: Handle<Image>,
    pub p2_body_turn_gray: Handle<Image>,
    pub p2_tail_gray: Handle<Image>,
    pub fruit_apple: Handle<Image>,
    pub fruit_strawberry: Handle<Image>,
    pub fruit_orange: Handle<Image>,
    pub fruit_grapes: Handle<Image>,
    pub fruit_banana: Handle<Image>,
    pub fruit_watermelon: Handle<Image>,
    pub fruit_starfruit: Handle<Image>,
    pub fruit_pepper: Handle<Image>,
}

impl SpriteAssets {
    pub fn snake_head(&self, player: PlayerId, is_gray: bool) -> Handle<Image> {
        match (player, is_gray) {
            (PlayerId::Player1, false) => self.p1_head.clone(),
            (PlayerId::Player1, true) => self.p1_head_gray.clone(),
            (PlayerId::Player2, false) => self.p2_head.clone(),
            (PlayerId::Player2, true) => self.p2_head_gray.clone(),
        }
    }

    pub fn snake_body(&self, player: PlayerId, is_gray: bool) -> Handle<Image> {
        self.snake_segment(player, SnakeSegmentKind::Straight, is_gray)
    }

    pub fn snake_segment(&self, player: PlayerId, kind: SnakeSegmentKind, is_gray: bool) -> Handle<Image> {
        match (player, kind, is_gray) {
            (PlayerId::Player1, SnakeSegmentKind::Straight, false) => self.p1_body.clone(),
            (PlayerId::Player1, SnakeSegmentKind::Straight, true) => self.p1_body_gray.clone(),
            (PlayerId::Player1, SnakeSegmentKind::Turn, false) => self.p1_body_turn.clone(),
            (PlayerId::Player1, SnakeSegmentKind::Turn, true) => self.p1_body_turn_gray.clone(),
            (PlayerId::Player1, SnakeSegmentKind::Tail, false) => self.p1_tail.clone(),
            (PlayerId::Player1, SnakeSegmentKind::Tail, true) => self.p1_tail_gray.clone(),
            (PlayerId::Player2, SnakeSegmentKind::Straight, false) => self.p2_body.clone(),
            (PlayerId::Player2, SnakeSegmentKind::Straight, true) => self.p2_body_gray.clone(),
            (PlayerId::Player2, SnakeSegmentKind::Turn, false) => self.p2_body_turn.clone(),
            (PlayerId::Player2, SnakeSegmentKind::Turn, true) => self.p2_body_turn_gray.clone(),
            (PlayerId::Player2, SnakeSegmentKind::Tail, false) => self.p2_tail.clone(),
            (PlayerId::Player2, SnakeSegmentKind::Tail, true) => self.p2_tail_gray.clone(),
        }
    }

    pub fn fruit_handle(&self, fruit_type: FruitType) -> Handle<Image> {
        match fruit_type {
            FruitType::Apple => self.fruit_apple.clone(),
            FruitType::Strawberry => self.fruit_strawberry.clone(),
            FruitType::Orange => self.fruit_orange.clone(),
            FruitType::Grapes => self.fruit_grapes.clone(),
            FruitType::Banana => self.fruit_banana.clone(),
            FruitType::Watermelon => self.fruit_watermelon.clone(),
            FruitType::Starfruit => self.fruit_starfruit.clone(),
            FruitType::HotPepper => self.fruit_pepper.clone(),
        }
    }
}

pub fn render_svg_to_image(svg_data: &[u8], width: u32, height: u32) -> Result<Image, String> {
    let opt = Options::default();
    let tree = Tree::from_data(svg_data, &opt).map_err(|e| e.to_string())?;

    let mut pixmap = Pixmap::new(width, height)
        .ok_or_else(|| "Failed to allocate pixmap".to_string())?;

    let sx = width as f32 / tree.size().width();
    let sy = height as f32 / tree.size().height();
    let transform = Transform::from_scale(sx, sy);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    Ok(Image::new(
        size,
        TextureDimension::D2,
        pixmap.take(),
        TextureFormat::Rgba8UnormSrgb,
        bevy::asset::RenderAssetUsages::default(),
    ))
}

pub fn render_svg_to_grayscale_image(svg_data: &[u8], width: u32, height: u32) -> Result<Image, String> {
    let opt = Options::default();
    let tree = Tree::from_data(svg_data, &opt).map_err(|e| e.to_string())?;

    let mut pixmap = Pixmap::new(width, height)
        .ok_or_else(|| "Failed to allocate pixmap".to_string())?;

    let sx = width as f32 / tree.size().width();
    let sy = height as f32 / tree.size().height();
    let transform = Transform::from_scale(sx, sy);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    for pixel in pixmap.data_mut().chunks_exact_mut(4) {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let gray = (0.299 * r + 0.587 * g + 0.114 * b).round().clamp(0.0, 255.0) as u8;
        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    }

    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };

    Ok(Image::new(
        size,
        TextureDimension::D2,
        pixmap.take(),
        TextureFormat::Rgba8UnormSrgb,
        bevy::asset::RenderAssetUsages::default(),
    ))
}

pub fn setup_svg_sprites(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // 128x128 resolution provides ultra-crisp antialiased rendering across all resolutions
    let tex_res: u32 = 128;

    let p1_head_svg = colorize_snake_svg(SVG_SNAKE_HEAD_TEMPLATE, PlayerId::Player1);
    let p1_straight_svg = colorize_snake_svg(SVG_SNAKE_BODY_STRAIGHT_TEMPLATE, PlayerId::Player1);
    let p1_turn_svg = colorize_snake_svg(SVG_SNAKE_BODY_TURN_TEMPLATE, PlayerId::Player1);
    let p1_tail_svg = colorize_snake_svg(SVG_SNAKE_TAIL_TEMPLATE, PlayerId::Player1);

    let p2_head_svg = colorize_snake_svg(SVG_SNAKE_HEAD_TEMPLATE, PlayerId::Player2);
    let p2_straight_svg = colorize_snake_svg(SVG_SNAKE_BODY_STRAIGHT_TEMPLATE, PlayerId::Player2);
    let p2_turn_svg = colorize_snake_svg(SVG_SNAKE_BODY_TURN_TEMPLATE, PlayerId::Player2);
    let p2_tail_svg = colorize_snake_svg(SVG_SNAKE_TAIL_TEMPLATE, PlayerId::Player2);

    let mut load_tex = |data: &[u8], gray: bool| -> Handle<Image> {
        let res = if gray {
            render_svg_to_grayscale_image(data, tex_res, tex_res)
        } else {
            render_svg_to_image(data, tex_res, tex_res)
        };
        res.map(|img| images.add(img)).unwrap_or_default()
    };

    let p1_head = load_tex(&p1_head_svg, false);
    let p1_body = load_tex(&p1_straight_svg, false);
    let p1_body_turn = load_tex(&p1_turn_svg, false);
    let p1_tail = load_tex(&p1_tail_svg, false);

    let p2_head = load_tex(&p2_head_svg, false);
    let p2_body = load_tex(&p2_straight_svg, false);
    let p2_body_turn = load_tex(&p2_turn_svg, false);
    let p2_tail = load_tex(&p2_tail_svg, false);

    let p1_head_gray = load_tex(&p1_head_svg, true);
    let p1_body_gray = load_tex(&p1_straight_svg, true);
    let p1_body_turn_gray = load_tex(&p1_turn_svg, true);
    let p1_tail_gray = load_tex(&p1_tail_svg, true);

    let p2_head_gray = load_tex(&p2_head_svg, true);
    let p2_body_gray = load_tex(&p2_straight_svg, true);
    let p2_body_turn_gray = load_tex(&p2_turn_svg, true);
    let p2_tail_gray = load_tex(&p2_tail_svg, true);

    let fruit_apple = load_tex(SVG_FRUIT_APPLE, false);
    let fruit_strawberry = load_tex(SVG_FRUIT_STRAWBERRY, false);
    let fruit_orange = load_tex(SVG_FRUIT_ORANGE, false);
    let fruit_grapes = load_tex(SVG_FRUIT_GRAPES, false);
    let fruit_banana = load_tex(SVG_FRUIT_BANANA, false);
    let fruit_watermelon = load_tex(SVG_FRUIT_WATERMELON, false);
    let fruit_starfruit = load_tex(SVG_FRUIT_STARFRUIT, false);
    let fruit_pepper = load_tex(SVG_FRUIT_PEPPER, false);

    commands.insert_resource(SpriteAssets {
        p1_head,
        p1_body,
        p1_body_turn,
        p1_tail,
        p2_head,
        p2_body,
        p2_body_turn,
        p2_tail,
        p1_head_gray,
        p1_body_gray,
        p1_body_turn_gray,
        p1_tail_gray,
        p2_head_gray,
        p2_body_gray,
        p2_body_turn_gray,
        p2_tail_gray,
        fruit_apple,
        fruit_strawberry,
        fruit_orange,
        fruit_grapes,
        fruit_banana,
        fruit_watermelon,
        fruit_starfruit,
        fruit_pepper,
    });
}

