use super::height_map::HeightMap;
use super::height_map::Neighbourhood;

pub fn build_slope_map(height_map: &HeightMap) -> HeightMap {
    let mut slope_map = HeightMap::with_edge_size(height_map.edge_size());
    for i in 0..height_map.edge_size() {
        for j in 0..height_map.edge_size() {
            *slope_map.at_mut(i, j) = height_map
                .get_neighbours(i, j, Neighbourhood::VonNeumann)
                .iter()
                .map(|neighbour| (neighbour - height_map.at(i, j)).abs())
                .fold(0f32, |max_slope, slope| max_slope.max(slope)); 
        }
    }
    slope_map
}

pub fn calculate_mean_value(map: &HeightMap) -> f32 {
    map.buffer().iter().sum::<f32>() / map.edge_size().pow(2) as f32
}

fn internal_standard_deviation(map: &HeightMap, mean_value: f32) -> f32 {
    map.buffer().iter()
        .map(|value| (mean_value - value).powi(2))
        .sum::<f32>().sqrt() / map.edge_size() as f32
}

pub fn calculate_standard_deviation(map: &HeightMap) -> f32 {
    let mean_value = calculate_mean_value(map);
    internal_standard_deviation(map, mean_value)
}

pub fn calculate_erosion_score(map: &HeightMap) -> f32 {
    let mean_value = calculate_mean_value(map);
    internal_standard_deviation(map, mean_value) / mean_value
}

#[cfg(test)]
mod tests {
    use super::*;

    static BUFFER: [f32; 16] = [
        01f32, 02f32, 03f32, 04f32,
        05f32, 06f32, 07f32, 08f32,
        09f32, 10f32, 11f32, 12f32,
        13f32, 14f32, 15f32, 16f32
    ];

    #[test]
    fn calculate_mean_value_test() {
        let height_map = HeightMap::from_buffer(&BUFFER).unwrap();
        assert_eq!(calculate_mean_value(&height_map), 8.5f32);
    }
}
