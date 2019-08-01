extern crate num;

use std::ops::BitAnd;
use num::{Num, Float};

/*
From this paper: http://web.mit.edu/cesium/Public/terrain.pdf
Slope map:
S[i][j] = max(
	|h[i][j] − h[i - 1][j]|,
	|h[i][j] − h[i + 1][j]|,
	|h[i][j] − h[i][j - 1]|,
	|h[i][j] − h[i][j + 1]|
) 
*/

pub struct HeightMap {
	map: Vec<f64>,
	side_size: usize
}

impl HeightMap {
	pub fn with_side_size(side_size: usize) -> Result<Self, String> {
		if !is_power_of_two(side_size) {
			return Err(format!("side size must be power of two, current is {}", side_size));
		}
		Ok(HeightMap{
			map: vec![0f64; side_size],
			side_size: side_size
		})
	}

	// TODO: add wrapping_at
	// TODO: check if x and y are out of bounds
	pub fn at(&self, x: usize, y: usize) -> f64 {
		self.map[self.side_size * y + x]
	}

	// TODO: add wrapping_at_mut
	// TODO: check if x and y are out of bounds
	pub fn at_mut(&mut self, x: usize, y: usize) -> &mut f64 {
		&mut self.map[self.side_size * y + x]
	}
}

pub fn build_slope_map(heigth_map: &HeightMap) {
	let mut slope_map = HeightMap::with_side_size(heigth_map.side_size).unwrap();
	for i in 0..heigth_map.side_size {
		for j in 0..heigth_map.side_size {
			let slopes = [
				num::abs(heigth_map.at(i, j) - heigth_map.at(i - 1, j)),
				num::abs(heigth_map.at(i, j) - heigth_map.at(i + 1, j)),
				num::abs(heigth_map.at(i, j) - heigth_map.at(i, j - 1)),
				num::abs(heigth_map.at(i, j) - heigth_map.at(i, j + 1)),
			];
			let slope_max = floats_max(&slopes).unwrap();
			*slope_map.at_mut(i, j) = *slope_max;
		}
	}
}

// TODO: floats_max must be able to process NaN values
// TODO: move floats_max to common::num module
fn floats_max<T>(slice: &[T]) -> Option<&T>
where T: Float {
	if slice.len() == 0 {
		return None;
	}
	slice.iter().max_by(|a, b| a.partial_cmp(b).unwrap())
}

// TODO: is_power_of_two to common::num module
fn is_power_of_two<T>(number: T) -> bool 
where T: Num + Copy + BitAnd<Output = T> {
	(number & (number - T::one())) == T::zero()
}
