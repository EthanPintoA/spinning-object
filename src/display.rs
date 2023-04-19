use std::fmt::Display;

use crate::draw::DisplayBuffer;

#[allow(dead_code)]
// http://paulbourke.net/dataformats/asciiart/
const GRAYSCALE_MINI: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
#[allow(dead_code)]
const GRAYSCALE_RECT: [char; 8] = [' ', '.', ':', '=', '░', '▒', '▓', '█'];
#[allow(dead_code)]
const GRAYSCALE_RECT2: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

fn luma_to_char(luma: u8) -> char {
	const ASCII_CHARS: [char; 10] = GRAYSCALE_MINI;
	const LAST_IDX: usize = ASCII_CHARS.len() - 1;

	// Converts (0.0 <= luma <= 255.0) to (0 <= idx <= LAST_IDX)
	let idx = (luma as f32 / 255.0 * LAST_IDX as f32).round() as usize;
	ASCII_CHARS[idx]
}

impl<const N: usize, const M: usize> Display for DisplayBuffer<N, M> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut print_buf = String::new();
		for row in self.0 {
			for luma in row {
				let char = luma_to_char(luma);
				print_buf.push(char);
				print_buf.push(char);
			}
			print_buf.push('\n');
		}

		write!(f, "{}", print_buf)
	}
}
