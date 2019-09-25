mod terrain;
use terrain::height_map::HeightMap;
use terrain::score::*;
use terrain::generation::*;
use terrain::render::*;

fn main() {
	let mut height_map = HeightMap::with_edge_size(512);
	let slope_map = build_slope_map(&height_map);
	calculate_mean_value(&slope_map);
    calculate_standard_deviation(&slope_map);

	diamond_square(&mut height_map);

	height_map.normalize();
	//println!("{:#?}", height_map);

	to_image(&height_map, "sucks.png").unwrap();
	to_text_file(&height_map, "cool.txt").unwrap();
}
