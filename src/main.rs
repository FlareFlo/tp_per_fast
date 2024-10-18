mod binary_search;
mod reader_thread;

use std::{
	fs,
	net::{Ipv6Addr, SocketAddrV6, TcpListener},
	os::fd::{AsFd, AsRawFd},
	sync::Arc,
	thread::sleep,
	time::{Duration, Instant},
};

use geo::{Contains, Coord, Coordinate, Geometry, Point, Rect};
use nix::sys::{socket, socket::sockopt::ReusePort};
use prost::Message;
use wkt::TryFromWkt;

use crate::{
	binary_search::BinarySearch,
	protobufs::{File, ProtobufBezirk},
	reader_thread::reader_thread,
};

pub mod protobufs {
	pub use Bezirk as ProtobufBezirk;
	include!(concat!(env!("OUT_DIR"), "/geodata.rs"));
	include!(concat!(env!("OUT_DIR"), "/wire.rs"));
}

pub struct BezirkLUT {
	raw:           Vec<ProtobufBezirk>,
	naive_linear:  Vec<Bezirk>,
	binary_search: BinarySearch,
}

#[derive(Clone, Debug)]
pub struct Bezirk {
	pub identifier: u64,
	pub parents:    Vec<u64>,
	pub name:       String,
	pub location:   Geometry,
}

impl BezirkLUT {
	pub fn new(b: File) -> Self {
		let parsed: Vec<_> = b
			.bezirke
			.iter()
			.map(|e| Bezirk {
				identifier: e.identifier,
				parents:    e.parents.clone(),
				name:       e.name.clone(),
				location:   Geometry::try_from_wkt_str(&e.wkt).unwrap(),
			})
			.collect();

		Self {
			binary_search: BinarySearch::new(
				10,
				Rect::new(
					Coord {
						x: 7.042,
						y: 53.745,
					},
					Coord {
						x: 14.019,
						y: 47.588,
					},
				),
				&parsed,
			),
			raw:           b.bezirke,
			naive_linear:  parsed,
		}
	}

	pub fn naive_lookup(&self, lat: f64, long: f64) -> Option<&str> {
		self.naive_linear
			.iter()
			.rev()
			.find(|e| e.location.contains(&Point::from((long, lat))))
			.map(|e| e.name.as_str())
	}

	pub fn binary_lookup(&self, lat: f64, long: f64) -> Option<&str> {
		Some(
			self.binary_search
				.lookup(Point::new(long, lat))?
				.name
				.as_str(),
		)
	}
}

fn main() {
	let bezirke = protobufs::File::decode(
		fs::read("/home/flareflo/tp_per/group-b/geodata/build/bezirke-12.geodata")
			.unwrap()
			.as_slice(),
	)
	.unwrap();
	let lut = Arc::new(BezirkLUT::new(bezirke));

	let port = 1234;
	let addr = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);
	let server = TcpListener::bind(addr).unwrap();
	socket::setsockopt(&server.as_fd(), ReusePort, &true).unwrap();

	// unsafe {
	//     signal_hook::low_level::signal(signal_hook::consts::SIGPIPE, signal_hook::low_level::SigHandler::SigIgn).unwrap();
	// }

	for stream in server.incoming() {
		match stream {
			Ok(socket) => {
				println!("new client");
				reader_thread(socket, lut.clone());
			},
			Err(e) => println!("couldn't get client: {e:?}"),
		}
	}
	sleep(Duration::from_secs(100))
}
