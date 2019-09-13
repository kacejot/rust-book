pub struct HeightMap {
    buffer: Vec<f32>,
    edge_size: usize
}

impl HeightMap {
    pub fn with_edge_size(edge_size: usize) -> Self {
        let edge_size = edge_size.next_power_of_two();
        println!("creating height map with egde size = {}", edge_size);
        HeightMap{
			buffer: vec![0f32; edge_size * edge_size],
			edge_size: edge_size
		}
    }

    pub fn from_buffer(buffer: &[f32]) -> Option<Self> {
        let edge = (buffer.len() as f32).sqrt() as usize;

        if !edge.is_power_of_two() {
            return None;
        }

        let mut result = Self::with_edge_size(edge);
        result.buffer = buffer.to_vec();
        Some(result)
    }

    pub fn at(&self, x: usize, y: usize) -> f32 {
		self.buffer[self.edge_size * y + x]
	}

    pub fn wrapping_at(&self, x: isize, y: isize) -> f32 {
        let x = wrap(x, self.edge_size);
        let y = wrap(y, self.edge_size);
        self.at(x, y)
    }

	pub fn at_mut(&mut self, x: usize, y: usize) -> &mut f32 {
		&mut self.buffer[self.edge_size * y + x]
	}

    pub fn edge_size(&self) -> usize {
        self.edge_size
    }

    pub fn buffer(&self) -> &[f32] {
        &self.buffer
    }
}

fn wrap(number: isize, edge_size: usize) -> usize {
    let mut number = number % edge_size as isize;
    if number < 0 {
        number = number + edge_size as isize;
    }
    number as usize
}
