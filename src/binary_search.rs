use geo::{Contains, Geometry, Intersects, Point, Rect};

use crate::Bezirk;

#[derive(Debug)]
pub struct BinarySearch {
	chunks: Vec<(Rect, Vec<Bezirk>)>,
}

impl BinarySearch {
	pub fn new(subdivisions: usize, starting_square: Rect, bezirke: &[Bezirk]) -> Self {
		let mut chunks = Vec::new();
		Self::from_square(starting_square, 0, bezirke, subdivisions, &mut chunks);
		Self { chunks }
	}

	fn from_square(
		square: Rect,
		i: usize,
		bezirke: &[Bezirk],
		limit: usize,
		chunks: &mut Vec<(Rect, Vec<Bezirk>)>,
	) {
		let horizontal = i.rem_euclid(2) == 0;

		// split square in half
		let [a, b] = if horizontal {
			square.split_x()
		} else {
			square.split_y()
		};

		if i >= limit {
			let in_chunk = bezirke
				.iter()
				.filter(|e| square.intersects(&e.location))
				.cloned()
				.collect();
			chunks.push((square, in_chunk));
		} else {
			Self::from_square(a, i + 1, bezirke, limit, chunks);
			Self::from_square(b, i + 1, bezirke, limit, chunks);
		}
	}

	pub fn lookup(&self, point: Point) -> Option<&Bezirk> {
		let valid_chunks = self
			.chunks
			.iter()
			.filter(|(rect, chunks)| rect.contains(&point))
			.map(|e| &e.1);
		for valid_chunk in valid_chunks {
			for bezirk in valid_chunk.iter().rev() {
				if bezirk.location.contains(&point) {
					return Some(&bezirk);
				}
			}
		}
		None
	}
}
