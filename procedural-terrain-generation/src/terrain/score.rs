use super::height_map::HeightMap;

pub fn build_slope_map(height_map: &HeightMap) -> HeightMap {
    let mut slope_map = HeightMap::with_edge_size(height_map.edge_size());
    for i in 0..height_map.edge_size() {
        for j in 0..height_map.edge_size() {
            let max_slope = get_slopes(i, j, height_map)
                .iter()
                .fold(0f32, |result, slope| slope.max(result));
            *slope_map.at_mut(i, j) = max_slope;
        }
    }
    slope_map
}

fn get_slopes(i: usize, j: usize, height_map: &HeightMap) -> [f32; 4] {
    let current = height_map.at(i, j);
    let i = i as isize;
    let j = j as isize;
    [
        (current - height_map.wrapping_at(i - 1, j)).abs(),
        (current - height_map.wrapping_at(i, j - 1)).abs(),
        (current - height_map.wrapping_at(i + 1, j)).abs(),
        (current - height_map.wrapping_at(i, j + 1)).abs()
    ]
}

pub fn calculate_mean_value(map: &HeightMap) -> f32 {
    map.buffer().iter().sum::<f32>() / map.edge_size().pow(2) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    static buffer: [f32; 16] = [
        01f32, 02f32, 03f32, 04f32,
        05f32, 06f32, 07f32, 08f32,
        09f32, 10f32, 11f32, 12f32,
        13f32, 14f32, 15f32, 16f32
    ];

    #[test]
    fn get_slopes_center() {
        let height_map = HeightMap::from_buffer(&buffer).unwrap();
        let slopes = get_slopes(1, 1, &height_map);
        assert_eq!(slopes.len(), 4);
        assert_eq!(slopes[0], 1f32);
        assert_eq!(slopes[1], 4f32);
        assert_eq!(slopes[2], 1f32);
        assert_eq!(slopes[3], 4f32);
    }

    #[test]
    fn get_slopes_top_left_corner() {
        let height_map = HeightMap::from_buffer(&buffer).unwrap();
        let slopes = get_slopes(0, 0, &height_map);
        assert_eq!(slopes.len(), 4);
        assert_eq!(slopes[0], 03f32);
        assert_eq!(slopes[1], 12f32);
        assert_eq!(slopes[2], 01f32);
        assert_eq!(slopes[3], 04f32);
    }

    #[test]
    fn get_slopes_lower_right_corner() {
        let height_map = HeightMap::from_buffer(&buffer).unwrap();
        let slopes = get_slopes(3, 3, &height_map);
        assert_eq!(slopes.len(), 4);
        assert_eq!(slopes[0], 01f32);
        assert_eq!(slopes[1], 04f32);
        assert_eq!(slopes[2], 03f32);
        assert_eq!(slopes[3], 12f32);
    }

    #[test]
    fn calculate_mean_value_empty() {
        let height_map = HeightMap::with_edge_size(0);
        assert_eq!(calculate_mean_value(&height_map), 0f32);
    }

    #[test]
    fn calculate_mean_value_test() {
        let height_map = HeightMap::from_buffer(&buffer).unwrap();
        assert_eq!(calculate_mean_value(&height_map), 8.5f32);
    }
}