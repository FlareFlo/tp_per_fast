use geo::{Contains, Point};
use crate::Bezirk;
use crate::geo_to_bezirk::GeoToBezirk;

pub struct NaiveLinear {
	bezirke:  Vec<Bezirk>,
}

impl GeoToBezirk for NaiveLinear {
	fn lookup(&self, long: f64, lat: f64) -> Option<&Bezirk> {
		self.bezirke
			.iter()
			.rev()
			.find(|e| e.location.contains(&Point::from((long, lat))))
	}
}