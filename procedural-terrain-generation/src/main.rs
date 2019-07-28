mod terrain;

use terrain::assessment;

fn main() {
	let _h = assessment::HeightMap::with_side_size(512).unwrap();
}
