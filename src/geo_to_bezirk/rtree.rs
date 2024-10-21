use geo::Point;
use geo::{BoundingRect, Coord, Rect};
use rstar::primitives::{GeomWithData, Rectangle};
use rstar::{RTree, AABB};
use crate::Bezirk;
use crate::geo_to_bezirk::GeoToBezirk;

type GeoWrapper = GeomWithData<Rectangle<(f64, f64)>, Bezirk>;

pub struct RStarTree {
	tree: RTree<GeoWrapper>,
}

impl RStarTree {
	pub fn new(bezirke: Vec<Bezirk>) -> Self {
		let bezirk_to_geo_wrapper = |e: Bezirk| {
			let [tl, _, br, _] = e.location.bounding_rect().unwrap().to_lines();
			let rect = Rectangle::from_corners(tl.start.x_y(), br.end.x_y());
			GeoWrapper::new(rect, e)
		};

		let tree = RTree::bulk_load(bezirke.into_iter().map(bezirk_to_geo_wrapper).collect());

		Self {
			tree
		}
	}
}

impl GeoToBezirk for RStarTree {
	fn lookup(&self, long: f64, lat: f64) -> Option<&Bezirk> {
		self.tree.locate_all_at_point(&(long, lat)).next().map(|e|&e.data)
	}
}