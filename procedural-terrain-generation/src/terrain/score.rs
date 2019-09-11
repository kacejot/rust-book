pub struct HeightMap {
    buffer: Vec<usize>,
    edge_size: usize
}

impl HeightMap {
    pub fn with_edge_size(edge_size: usize) -> Self {
        let edge_size = edge_size.next_power_of_two();
        println!("creating height map with egde size = {}", edge_size);
        HeightMap{
			buffer: vec![0; edge_size * edge_size],
			edge_size: edge_size
		}
    }

    pub fn at(&self, x: usize, y: usize) -> usize {
        let x = wrap(x, self.edge_size);
        let y = wrap(y, self.edge_size);
		self.buffer[self.edge_size * y + x]
	}


	pub fn at_mut(&mut self, x: usize, y: usize) -> &mut usize {
        let x = wrap(x, self.edge_size);
        let y = wrap(y, self.edge_size);
		&mut self.buffer[self.edge_size * y + x]
	}

    pub fn edge_size(&self) -> usize {
        self.edge_size
    }
}

enum Neighbourhood {
    VonNeumann,
    Moore
}

fn build_slope_map(height_map: &HeightMap, neighbourhood: Neighbourhood) {
    for i in (0..height_map.edge_size()) {
        for j in (0..height_map.edge_size()) {
            let max_slope = get_neighbours(i, j, height_map, neighbourhood).
                iter().max();
        }
    }
}

fn get_neighbours(i: usize, j: usize, height_map: &HeightMap, neighbourhood: Neighbourhood) -> Vec<usize> {
    match neighbourhood {
        VonNeumann => vec![
            height_map.at(i - 1, j),
            height_map.at(i + 1, j),
            height_map.at(i, j - 1),
            height_map.at(i, j + 1)],
        Moore => vec![
            height_map.at(i - 1, j),
            height_map.at(i + 1, j),
            height_map.at(i, j - 1),
            height_map.at(i, j + 1),
            height_map.at(i - 1, j - 1),
            height_map.at(i + 1, j - 1),
            height_map.at(i - 1, j + 1),
            height_map.at(i + 1, j + 1)
        ]
    }
}

fn wrap(number: usize, edge_size: usize) -> usize {
    let number = number % edge_size;
    if number < 0 {
        let number = number + edge_size;
    }
    number
}