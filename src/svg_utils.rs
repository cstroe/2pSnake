use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use resvg::usvg::{Options, Tree};
use resvg::tiny_skia::{Pixmap, Transform};

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
