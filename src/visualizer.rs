use std::fs;

use geo::{Coord, CoordsIter, Geometry, MultiPolygon};
use image::{ImageBuffer, Rgb};
use image::imageops::{flip_vertical, interpolate_bilinear};
use imageproc::drawing::{draw_antialiased_line_segment_mut, draw_line_segment_mut, Blend};
use imageproc::pixelops::interpolate;
use petgraph::visit::Walker;
use prost::Message;
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
	let img_height = 8000;
	let img_width = (((min_lon / max_lon) * img_height as f64)) as u32;
	//let img_width = img_height;
	let mut img = ImageBuffer::from_fn(img_width, img_height, |_x, _y| {
		Rgb([28, 28, 28]) // White background
	});

	// Load the GeoJSON file and extract the features
	for bezirk in parsed {
		draw_polygons(&mut img, bezirk);
	}
	img = flip_vertical(&img);
	// Save the image
	img.save("map.png").unwrap();
}

fn draw_polygons(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, bezirk: Bezirk) {
	let geo: &MultiPolygon= &bezirk.location.try_into().unwrap();
	for polygon in geo.iter() {
		for arr in polygon.exterior().lines().collect::<Vec<_>>().windows(2) {
			let old = arr[1];
			let new = arr[0];
			let (x1, y1) = scale_coordinates(old.end.x, old.end.y, image.width(), image.height());
			let (x2, y2) =
				scale_coordinates(new.start.x, new.start.y, image.width(), image.height());

			let start = (y1, x1);
			let end = (y2, x2);
			let colrs = [Rgb([255,255,255]), Rgb([255, 87, 20]), Rgb([232, 170, 20]), Rgb([228, 255, 26]), Rgb([110, 235, 131]), Rgb([27, 231, 255]), Rgb([255,27,117]), Rgb([255,51,27])];
			let color = colrs[bezirk.parents.len()];
			draw_antialiased_line_segment_mut(image, start, end, color, interpolate);
		}
	}
}

fn scale_coordinates(lat: f64, lon: f64, img_width: u32, img_height: u32) -> (i32, i32) {
	let x = (((lon - min_lon) / (max_lon - min_lon)).abs() * img_height as f64);
	let y = (((lat - min_lat) / (max_lat - min_lat)).abs() * img_width as f64);
	(x as i32, y as i32)
}
