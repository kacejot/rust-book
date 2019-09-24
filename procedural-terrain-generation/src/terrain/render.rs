extern crate image;

use super::height_map::HeightMap;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

pub fn to_image(height_map: &HeightMap, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let encoder = PNGEncoder::new(file);
    encoder.encode(
        &height_map.to_byte(),
        height_map.edge_size() as u32,
        height_map.edge_size() as u32,
        ColorType::Gray(8))?;
    Ok(())
}