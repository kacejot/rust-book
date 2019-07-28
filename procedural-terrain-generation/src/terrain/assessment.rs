extern crate num;

use num_traits::identities::{One, Zero};
use std::ops::{Sub, BitAnd};
use std::cmp;

/* 

Slope map:
S[i][j] = max(|h[i][j] − h[i - 1][j]|,
			  |h[i][j] − h[i + 1][j]|,
			  |h[i][j] − h[i][j - 1]|,
			  |h[i][j] − h[i][j + 1]|) 

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

	pub fn at(&self, x: usize, y: usize) -> f64 {
		self.map[self.side_size * y + x]
	}

	pub fn at_mut(&mut self, x: usize, y: usize) -> &mut f64 {
		&mut self.map[self.side_size * y + x]
	}
}

pub fn build_slope_map(heigth_map: &HeightMap, side_size: usize) {
	let mut slope_map = HeightMap::with_side_size(side_size).unwrap();
	for i in 0..side_size {
		for j in 0..side_size {
			// TODO: can't calculate max for floats (Ord trait must be implemented)
			*slope_map.at_mut(i, j) = [
				num::abs(heigth_map.at(i, j) - heigth_map.at(i - 1, j)),
				num::abs(heigth_map.at(i, j) - heigth_map.at(i + 1, j)),
				num::abs(heigth_map.at(i, j) - heigth_map.at(i, j - 1)),
				num::abs(heigth_map.at(i, j) - heigth_map.at(i, j + 1)),
			].iter().max().unwrap();
		}
	}
}

fn is_power_of_two<T>(number: T) -> bool 
where T: Sub<Output = T> + BitAnd<Output = T> + One + Zero + PartialEq + Copy {
	return (number & (number - T::one())) == T::zero()
}
