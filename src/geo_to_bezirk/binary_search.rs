use geo::{Contains, Coord, Geometry, Intersects, Point, Rect};

use crate::{Bezirk, BezirkeData};
use crate::geo_to_bezirk::GeoToBezirk;

#[derive(Debug)]
#[derive(Clone)]
pub struct BinarySearch {
	chunks: Vec<(Rect, Vec<Bezirk>)>,
}

impl GeoToBezirk for BinarySearch {
	fn lookup(&self, lat: f64, long: f64) -> Option<&Bezirk> {
		let point = Point::new(long, lat);
		let valid_chunks = self
			.chunks
			.iter()
			.filter(|(rect, _)| rect.contains(&point))
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

impl BinarySearch {
	pub fn new(subdivisions: usize, starting_square: Rect, bezirke: &[Bezirk]) -> Self {
		let mut chunks = Vec::new();
		Self::from_square(starting_square, 0, bezirke, subdivisions, &mut chunks);
		Self { chunks }
	}

	pub fn new_with_defaults(subdivisions: usize, bezirke: BezirkeData) -> Self {
		Self::new(subdivisions, Rect::new(
					Coord {
						x: 7.042,
						y: 53.745,
					},
					Coord {
						x: 14.019,
						y: 47.588,
					},
				), bezirke.data.as_slice())
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
}
