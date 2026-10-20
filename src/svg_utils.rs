use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use resvg::usvg::{Options, Tree};
use resvg::tiny_skia::{Pixmap, Transform};
use crate::types::*;

const SVG_SNAKE_HEAD_P1: &[u8] = include_bytes!("../assets/svg/snake_head_p1.svg");
const SVG_SNAKE_HEAD_P2: &[u8] = include_bytes!("../assets/svg/snake_head_p2.svg");
const SVG_SNAKE_BODY_P1: &[u8] = include_bytes!("../assets/svg/snake_body_p1.svg");
const SVG_SNAKE_BODY_P2: &[u8] = include_bytes!("../assets/svg/snake_body_p2.svg");
const SVG_FRUIT_APPLE: &[u8] = include_bytes!("../assets/svg/fruit_apple.svg");
const SVG_FRUIT_STRAWBERRY: &[u8] = include_bytes!("../assets/svg/fruit_strawberry.svg");
const SVG_FRUIT_ORANGE: &[u8] = include_bytes!("../assets/svg/fruit_orange.svg");
const SVG_FRUIT_GRAPES: &[u8] = include_bytes!("../assets/svg/fruit_grapes.svg");
const SVG_FRUIT_BANANA: &[u8] = include_bytes!("../assets/svg/fruit_banana.svg");
const SVG_FRUIT_WATERMELON: &[u8] = include_bytes!("../assets/svg/fruit_watermelon.svg");
const SVG_FRUIT_STARFRUIT: &[u8] = include_bytes!("../assets/svg/fruit_starfruit.svg");
const SVG_FRUIT_PEPPER: &[u8] = include_bytes!("../assets/svg/fruit_pepper.svg");

#[derive(Resource, Default, Clone)]
pub struct SpriteAssets {
    pub p1_head: Handle<Image>,
    pub p1_body: Handle<Image>,
    pub p2_head: Handle<Image>,
    pub p2_body: Handle<Image>,
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

pub fn setup_svg_sprites(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    // 128x128 resolution provides ultra-crisp antialiased rendering across all resolutions
    let tex_res: u32 = 128;

    let p1_head = render_svg_to_image(SVG_SNAKE_HEAD_P1, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    let p1_body = render_svg_to_image(SVG_SNAKE_BODY_P1, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    let p2_head = render_svg_to_image(SVG_SNAKE_HEAD_P2, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    let p2_body = render_svg_to_image(SVG_SNAKE_BODY_P2, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    let fruit_apple = render_svg_to_image(SVG_FRUIT_APPLE, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    let fruit_strawberry = render_svg_to_image(SVG_FRUIT_STRAWBERRY, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    let fruit_orange = render_svg_to_image(SVG_FRUIT_ORANGE, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    let fruit_grapes = render_svg_to_image(SVG_FRUIT_GRAPES, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    let fruit_banana = render_svg_to_image(SVG_FRUIT_BANANA, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    let fruit_watermelon = render_svg_to_image(SVG_FRUIT_WATERMELON, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    let fruit_starfruit = render_svg_to_image(SVG_FRUIT_STARFRUIT, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    let fruit_pepper = render_svg_to_image(SVG_FRUIT_PEPPER, tex_res, tex_res)
        .map(|img| images.add(img))
        .unwrap_or_default();

    commands.insert_resource(SpriteAssets {
        p1_head,
        p1_body,
        p2_head,
        p2_body,
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
