mod terrain;

use terrain::assessment;

fn main() {
	let h = assessment::HeightMap::with_side_size(512).unwrap();
	assessment::build_slope_map(&h);
}
