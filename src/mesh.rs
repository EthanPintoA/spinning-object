use glam::Vec3;

pub struct Triangle {
	vertices: [Vec3; 3],
}

impl Triangle {
	/// Creates a new triangle.
	pub fn new(vertices: [Vec3; 3]) -> Self {
		Self { vertices }
	}

	/// Returns an array of each edge's start and end position.
	pub fn get_edges(&self) -> [(&Vec3, &Vec3); 3] {
		[
			(&self.vertices[0], &self.vertices[1]),
			(&self.vertices[0], &self.vertices[2]),
			(&self.vertices[1], &self.vertices[2]),
		]
	}
}
