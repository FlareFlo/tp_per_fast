use std::fs;

use geo::{Coord, CoordsIter, Geometry, MultiPolygon};
use image::{ImageBuffer, Rgb};
use image::imageops::flip_vertical;
use imageproc::drawing::draw_line_segment_mut;
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
		Rgb([255, 255, 255]) // White background
	});

	// Load the GeoJSON file and extract the features
	for bezirk in parsed {
		draw_polygons(&mut img, bezirk.location.try_into().unwrap());
	}
	img = flip_vertical(&img);
	// Save the image
	img.save("map.png").unwrap();
}

fn draw_polygons(image: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, geodata: MultiPolygon) {
	for polygon in geodata.iter() {
		for arr in polygon.exterior().lines().collect::<Vec<_>>().windows(2) {
			let old = arr[1];
			let new = arr[0];
			let (x1, y1) = scale_coordinates(old.end.x, old.end.y, image.width(), image.height());
			let (x2, y2) =
				scale_coordinates(new.start.x, new.start.y, image.width(), image.height());

			let start = (y1, x1);
			let end = (y2, x2);
			let color = Rgb([0, 0, 0]);
			draw_line_segment_mut(image, start, end, color);
		}
	}
}

fn scale_coordinates(lat: f64, lon: f64, img_width: u32, img_height: u32) -> (f32, f32) {
	let x = (((lon - min_lon) / (max_lon - min_lon)).abs() * img_height as f64);
	let y = (((lat - min_lat) / (max_lat - min_lat)).abs() * img_width as f64);
	(x as f32, y as f32)
}
