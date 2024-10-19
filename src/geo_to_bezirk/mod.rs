use crate::Bezirk;

pub mod binary_search;
mod naive_linear;

pub trait GeoToBezirk where Self: Sized {
	fn lookup(&self, long: f64, lat: f64) -> Option<&Bezirk>;
}