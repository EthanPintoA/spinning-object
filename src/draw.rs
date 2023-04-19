use std::collections::HashMap;
use std::f32::consts::PI;

use glam::{IVec2, Vec2Swizzles, Vec3};

use crate::mesh::Triangle;

pub struct DisplayBuffer<const N: usize, const M: usize>(pub [[u8; M]; N]);

impl<const N: usize, const M: usize> DisplayBuffer<N, M> {
	pub fn draw_mesh(&mut self, mesh: &[Triangle], light_direction: Vec3) {
		let mut z_buf = [[f32::MAX; M]; N];

		for triangle in mesh {
			// Using `avg` as the pixels' z-value because it's easier than calculating the
			// z-value for every pixel.
			let avg_z = triangle.get_center().z;
			let brightness = get_brightness(triangle.get_normal(), light_direction);

			for pixel in get_triangle_pixels(triangle) {
				if !(0..N).contains(&(pixel.y as usize)) {
					continue;
				}
				if !(0..M).contains(&(pixel.x as usize)) {
					continue;
				}
				if z_buf[pixel.y as usize][pixel.x as usize] < avg_z {
					continue;
				}
				z_buf[pixel.y as usize][pixel.x as usize] = avg_z;
				self.0[pixel.y as usize][pixel.x as usize] = brightness;
			}
		}
	}
}

/// Get how bright the surface is given a directional light.
fn get_brightness(surface_normal: Vec3, light_direction: Vec3) -> u8 {
	const HALF_PI: f32 = PI / 2.0;

	let angle_diff = surface_normal.angle_between(light_direction);
	// Force triangle to always face camera (due to no back-face culling)
	let angle_diff = if angle_diff < HALF_PI {
		PI - angle_diff
	} else {
		angle_diff
	};

	// Converts (HALF_PI <= angle_diff <= PI) to (0 <= brightness<= 255)
	(255.0 * (angle_diff - HALF_PI) / HALF_PI).round() as u8
}

/// Returns vector positions dependent positions
///
/// `i` is for independent `d` is for dependent.
///
/// Example:
/// ```
/// let (x0, y0) = (0, 0);
/// let (x1, y1) = (1, 2);
///
/// let positions = if (y1 - y0).abs() < (x1 - x0).abs() {
///     // Slope is less than 1.0
///     interpolate(x0, y0, x1, y1);
/// } else {
///     interpolate(x0, y0, x1, y1);
/// }
/// ```
///
/// Source: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
fn interpolate_line(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<(i32, i32)> {
	if i0 == i1 {
		return vec![(i0, d0)];
	}

	// Make sure i1 > i0
	let (i0, d0, i1, d1) = if i1 < i0 {
		(i1, d1, i0, d0)
	} else {
		(i0, d0, i1, d1)
	};

	let slope = (d1 - d0) as f32 / (i1 - i0) as f32;

	let get_d = |i| slope * i + d0 as f32;

	(i0..=i1)
		.enumerate()
		.map(|(idx, i)| (i, get_d(idx as f32) as i32))
		.collect()
}

/// Returns vector of positions representing the line on a raster.
fn get_bresenhams_line(p0: IVec2, p1: IVec2) -> Vec<IVec2> {
	if (p1.y - p0.y).abs() <= (p1.x - p0.x).abs() {
		// If slope is less than 1.0, independent is x
		interpolate_line(p0.x, p0.y, p1.x, p1.y)
			.into_iter()
			.map(IVec2::from)
			.collect()
	} else {
		// If slope is greater than 1.0, independent is y
		interpolate_line(p0.y, p0.x, p1.y, p1.x)
			.into_iter()
			.map(IVec2::from)
			.map(Vec2Swizzles::yx)
			.collect()
	}
}

fn get_triangle_pixels(triangle: &Triangle) -> Vec<IVec2> {
	let mut pixel_edges_map = HashMap::<_, Vec<_>>::new();

	triangle
		.get_edges()
		.iter()
		.map(|(v0, v1)| (v0.truncate(), v1.truncate()))
		.map(|(v0, v1)| (v0.round().as_ivec2(), v1.round().as_ivec2()))
		.flat_map(|(v0, v1)| get_bresenhams_line(v0, v1))
		.for_each(|ivec| pixel_edges_map.entry(ivec.y).or_default().push(ivec.x));

	pixel_edges_map
		.iter()
		.flat_map(|(y, x_vec)| {
			let min = *x_vec.iter().min().unwrap();
			let max = *x_vec.iter().max().unwrap();
			(min..=max).map(|x| IVec2::new(x, *y))
		})
		.collect()
}
