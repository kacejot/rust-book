mod terrain;
use terrain::height_map::HeightMap;
use terrain::score::*;

fn main() {
	let h = HeightMap::with_edge_size(511);
	let _sm = build_slope_map(&h);
}
