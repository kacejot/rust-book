mod terrain;
use terrain::height_map::HeightMap;
use terrain::score::*;

static BUFFER: [f32; 16] = [
	01f32, 02f32, 03f32, 04f32,
	05f32, 06f32, 07f32, 08f32,
	09f32, 10f32, 11f32, 12f32,
	13f32, 14f32, 15f32, 16f32
];

fn main() {
	let height_map = HeightMap::with_edge_size(511);
	let slope_map = build_slope_map(&height_map);
	let _m = calculate_mean_value(&slope_map);
	let _h = HeightMap::from_buffer(&BUFFER);
}
