extern crate image;

use super::height_map::HeightMap;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::io::Write;

pub fn to_image(height_map: &HeightMap, filename: &str) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let encoder = PNGEncoder::new(file);
    encoder.encode(
        &height_map.to_bytes(),
        height_map.edge_size() as u32,
        height_map.edge_size() as u32,
        ColorType::Gray(8))?;
    Ok(())
}

pub fn to_text_file(height_map: &HeightMap, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for i in 0..height_map.edge_size() {
        for j in 0..height_map.edge_size() {
            write!(file, "{:3.2} ", height_map.at(i, j));
        }
        writeln!(file, "");
    }

    Ok(())
}