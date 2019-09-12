use super::height_map::HeightMap;

pub fn build_slope_map(height_map: &HeightMap) {
    let mut slope_map = HeightMap::with_edge_size(height_map.edge_size());
    for i in 0..height_map.edge_size() {
        for j in 0..height_map.edge_size() {
            let max_slope = get_slopes(i, j, height_map)
                .iter()
                .fold(0f32, |result, slope| slope.max(result));
            *slope_map.at_mut(i, j) = max_slope;
        }
    }
}

fn get_slopes(i: usize, j: usize, height_map: &HeightMap) -> [f32; 4] {
    let current = height_map.at(i, j);
    let i = i as isize;
    let j = j as isize;
    [
        (current - height_map.wrapping_at(i - 1, j)).abs(),
        (current - height_map.wrapping_at(i + 1, j)).abs(),
        (current - height_map.wrapping_at(i, j - 1)).abs(),
        (current - height_map.wrapping_at(i, j + 1)).abs()
    ]
}