#[derive(Debug, Clone)]
pub struct HeightMap {
    buffer: Vec<f32>,
    edge_size: usize
}

pub enum Neighbourhood {
    Moore,
    VonNeumann
}

impl HeightMap {
    pub fn with_edge_size(edge_size: usize) -> Self {
        let edge_size = edge_size.next_power_of_two();
        HeightMap{
			buffer: vec![0f32; edge_size * edge_size],
			edge_size: edge_size
		}
    }

    pub fn at(&self, x: usize, y: usize) -> f32 {
		self.buffer[self.edge_size * y + x]
	}

	pub fn at_mut(&mut self, x: usize, y: usize) -> &mut f32 {
		&mut self.buffer[self.edge_size * y + x]
	}
    
    pub fn wrapping_at(&self, x: isize, y: isize) -> f32 {
        let x = wrap(x, self.edge_size);
        let y = wrap(y, self.edge_size);
        self.at(x, y)
    }

    pub fn wrapping_at_mut(&mut self, x: isize, y: isize) -> &mut f32 {
        let x = wrap(x, self.edge_size);
        let y = wrap(y, self.edge_size);
        self.at_mut(x, y)
    }

    pub fn edge_size(&self) -> usize {
        self.edge_size
    }

    pub fn buffer(&self) -> &[f32] {
        &self.buffer
    }

    pub fn get_neighbours(&self, i: usize, j: usize, neighbourhood: Neighbourhood) -> Vec<f32> {
        let i = i as isize;
        let j = j as isize;
        match neighbourhood {
            Neighbourhood::VonNeumann => vec![
                self.wrapping_at(i - 1, j),
                self.wrapping_at(i + 1, j),
                self.wrapping_at(i, j - 1),
                self.wrapping_at(i, j + 1)],
            Neighbourhood::Moore => vec![
                self.wrapping_at(i - 1, j),
                self.wrapping_at(i + 1, j),
                self.wrapping_at(i, j - 1),
                self.wrapping_at(i, j + 1),
                self.wrapping_at(i - 1, j - 1),
                self.wrapping_at(i + 1, j - 1),
                self.wrapping_at(i - 1, j + 1),
                self.wrapping_at(i + 1, j + 1)
            ]
        }
    }

    pub fn normalize(&mut self) {
        let max = *self.buffer.iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();

        let min = *self.buffer.iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap();

        let gap = max - min;
        for height in &mut self.buffer {
            *height = (*height - min) / gap;
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut height_map = self.clone();
        height_map.normalize();
        height_map.buffer().iter()
            .map(|value| (value * u8::max_value() as f32) as u8)
            .collect()
    }
}

fn wrap(number: isize, edge_size: usize) -> usize {
    let mut number = number % edge_size as isize;
    if number < 0 {
        number = number + edge_size as isize;
    }
    number as usize
}
