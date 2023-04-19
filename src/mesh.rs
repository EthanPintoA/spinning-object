use glam::{Quat, Vec3};

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

	/// Rotate about `pos`.
	pub fn rotate(&mut self, quat: Quat, pos: Vec3) {
		for v in self.vertices.iter_mut() {
			*v = (quat * (*v - pos)) + pos;
		}
	}

	pub fn get_normal(&self) -> Vec3 {
		let v0 = self.vertices[1] - self.vertices[0];
		let v1 = self.vertices[2] - self.vertices[1];
		(v0).cross(v1).normalize_or_zero()
	}

	pub fn get_center(&self) -> Vec3 {
		(self.vertices[0] + self.vertices[1] + self.vertices[2]) / 3.0
	}
}

/// Rotate about `pos`.
pub fn rotate_mesh(mesh: &mut [Triangle], quat: Quat, pos: Vec3) {
	for triangle in mesh {
		triangle.rotate(quat, pos);
	}
}

/// Returns triangles consisting of `[v1, v2, v3]` and `[v2, v3, v4]`
fn from_quad(v1: Vec3, v2: Vec3, v3: Vec3, v4: Vec3) -> [Triangle; 2] {
	[Triangle::new([v1, v2, v3]), Triangle::new([v2, v3, v4])]
}

/// Create an cube mesh
/// `position` is the center of the cube.
/// `size` is the length of a cube's side.
pub fn get_cube_mesh(pos: Vec3, size: f32) -> Vec<Triangle> {
	// Common values
	let left_x = pos.x - (size / 2.0);
	let right_x = pos.x + (size / 2.0);
	let top_y = pos.y + (size / 2.0);
	let bottom_y = pos.y - (size / 2.0);
	let front_z = pos.z - (size / 2.0);
	let back_z = pos.z + (size / 2.0);

	// Common vertexes
	let left_top_front = Vec3::new(left_x, top_y, front_z);
	let left_top_back = Vec3::new(left_x, top_y, back_z);
	let left_bottom_front = Vec3::new(left_x, bottom_y, front_z);
	let left_bottom_back = Vec3::new(left_x, bottom_y, back_z);
	let right_top_front = Vec3::new(right_x, top_y, front_z);
	let right_top_back = Vec3::new(right_x, top_y, back_z);
	let right_bottom_front = Vec3::new(right_x, bottom_y, front_z);
	let right_bottom_back = Vec3::new(right_x, bottom_y, back_z);

	// Faces
	let left = from_quad(
		left_top_back,
		left_top_front,
		left_bottom_back,
		left_bottom_front,
	);
	let right = from_quad(
		right_top_front,
		right_top_back,
		right_bottom_front,
		right_bottom_back,
	);
	let top = from_quad(
		left_top_back,
		right_top_back,
		left_top_front,
		right_top_front,
	);
	let bottom = from_quad(
		left_bottom_front,
		right_bottom_front,
		left_bottom_back,
		right_bottom_back,
	);
	let back = from_quad(
		right_top_back,
		left_top_back,
		right_bottom_back,
		left_bottom_back,
	);
	let front = from_quad(
		left_top_front,
		right_top_front,
		left_bottom_front,
		right_bottom_front,
	);

	let mut mesh = Vec::with_capacity(6 * 2);
	mesh.extend(left);
	mesh.extend(right);
	mesh.extend(top);
	mesh.extend(bottom);
	mesh.extend(back);
	mesh.extend(front);

	mesh
}
