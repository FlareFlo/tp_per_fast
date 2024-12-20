use std::{fs, ops::Mul};

use geo::{Geometry, MultiPolygon};
use svg::{
	node::element::{Group, Polygon},
	Document,
};
use wkt::TryFromWkt;
use protodefs::from_env;

#[derive(Clone, Debug)]
pub struct Bezirk {
	pub identifier: u64,
	pub parents:    Vec<u64>,
	pub name:       String,
	pub location:   Geometry,
}

static MIN_LON: f64 = 47.2;
static MIN_LAT: f64 = 5.8;
static MAX_LON: f64 = 55.2;
static MAX_LAT: f64 = 15.2;

static SCALE: f64 = 1.0;

fn main() {
	let bezirke = from_env();

	let parsed: Vec<_> = bezirke
		.bezirke
		.iter()
		.map(|e| Bezirk {
			identifier: e.identifier,
			parents:    e.parents.clone(),
			name:       e.name.clone(),
			location:   Geometry::try_from_wkt_str(&e.wkt).unwrap(),
		})
		.collect();
	let mut document = Document::new()
		.set(
			"viewBox",
			format!(
				"{} {} {} {}",
				MIN_LAT.mul(SCALE) as i32,
				MIN_LON.mul(SCALE) as i32,
				MAX_LAT.mul(SCALE) as i32,
				MAX_LON.mul(SCALE) as i32
			),
		)
		.set("transform", "scale(1, -1)");

	// Load the GeoJSON file and extract the features
	for bezirk in parsed {
		document = draw_polygons(document, bezirk);
	}
	let mut res = Vec::with_capacity(30_000_000);
	svg::write(&mut res, &document).unwrap();

	fs::write("multipolygon.svg", &res).unwrap()
}

fn draw_polygons(mut document: Document, bezirk: Bezirk) -> Document {
	let geo: &MultiPolygon = &bezirk.location.try_into().unwrap();
	let colors = [
		"black",
		"hotpink",
		"orchid",
		"violet",
		"palevioletred",
		"pink",
		"plum",
		"thistle",
	];
	for polygon in geo.iter() {
		let svg_polygon = Polygon::new()
			.set("fill", "none")
			.set("stroke", colors[bezirk.parents.len()])
			.set("stroke-width", 0.01 * SCALE)
			.set(
				"points",
				polygon
					.exterior()
					.points()
					.map(|p| format!("{},{}", p.x() * SCALE, p.y() * SCALE))
					.collect::<Vec<String>>()
					.join(" "),
			);

		document = document.add(Group::new().add(svg_polygon));
	}
	document
}
