use svg::node::element::{Group, Polygon};
use std::fs;

use geo::{Coord, CoordsIter, Geometry, MultiPolygon};
use image::{ImageBuffer, Rgb};
use image::imageops::{flip_vertical, interpolate_bilinear};
use imageproc::drawing::{draw_antialiased_line_segment_mut, draw_line_segment_mut, Blend};
use imageproc::pixelops::interpolate;
use petgraph::visit::Walker;
use prost::Message;
use svg::Document;
use svg::node::element::Path;
use wkt::TryFromWkt;

#[derive(Clone, Debug)]
pub struct Bezirk {
	pub identifier: u64,
	pub parents:    Vec<u64>,
	pub name:       String,
	pub location:   Geometry,
}

pub mod protobufs {
	pub use Bezirk as ProtobufBezirk;
	include!(concat!(env!("OUT_DIR"), "/geodata.rs"));
	include!(concat!(env!("OUT_DIR"), "/wire.rs"));
}

static min_lon: f64 = 47.2;
static min_lat: f64 = 5.8;
static max_lon: f64 = 55.2;
static max_lat: f64 = 15.2;

fn main() {
	let bezirke = protobufs::File::decode(
		fs::read("/home/flareflo/tp_per/group-b/geodata/result/geodata/bezirke-12.geodata")
			.unwrap()
			.as_slice(),
	)
	.unwrap();

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
	let mut document = Document::new().set("viewBox", "0 0 100 100");

	// Load the GeoJSON file and extract the features
	for bezirk in parsed {
		document = draw_polygons(document, bezirk);
	}
	    svg::save("multipolygon.svg", &document).expect("Failed to save SVG file");
}

fn draw_polygons(mut document: Document, bezirk: Bezirk) -> Document {
	let geo: &MultiPolygon= &bezirk.location.try_into().unwrap();
	    for polygon in geo.iter() {
        let svg_polygon = Polygon::new()
            .set("fill", "blue")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("points", polygon.exterior().points().map(|p| {
                format!("{},{}", p.x(), p.y())
            }).collect::<Vec<String>>().join(" "));

        document = document.add(Group::new().add(svg_polygon));
    }
	document
}