use glam::{IVec2, Vec2, Vec2Swizzles};

const BUF_WIDTH: usize = 26;
const BUF_HEIGHT: usize = 26;

struct DisplayBuffer<const N: usize, const M: usize>(pub [[u8; M]; N]);

fn main() {
	let mut display_buf = DisplayBuffer([[0; BUF_WIDTH]; BUF_HEIGHT]);

	let mesh = [
		[
			Vec2::new(1.0, 1.0),
			Vec2::new(22.0, 4.0),
			Vec2::new(4.0, 22.0),
		],
		[
			Vec2::new(22.0, 4.0),
			Vec2::new(4.0, 22.0),
			Vec2::new(22.0, 20.0),
		],
	];

	for triangle in mesh {
		for pixel in get_triangle_pixels(&triangle) {
			if !(0..BUF_HEIGHT).contains(&(pixel.y as usize)) {
				return;
			}
			if !(0..BUF_WIDTH).contains(&(pixel.x as usize)) {
				return;
			}
			display_buf.0[pixel.y as usize][pixel.x as usize] = u8::MAX;
		}
	}

	let mut print_buf = String::new();
	for row in display_buf.0 {
		for val in row {
			let pixel = if val > 128 { '#' } else { ' ' };
			print_buf.push(pixel);
			print_buf.push(pixel);
		}
		print_buf.push('\n');
	}

	println!("{}", print_buf);
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

fn get_triangle_pixels(triangle: &[Vec2; 3]) -> Vec<IVec2> {
	let edges = [
		(triangle[0], triangle[1]),
		(triangle[0], triangle[2]),
		(triangle[1], triangle[2]),
	];

	edges
		.into_iter()
		.map(|(v0, v1)| (v0.round().as_ivec2(), v1.round().as_ivec2()))
		.flat_map(|(v0, v1)| get_bresenhams_line(v0, v1))
		.collect()
}
