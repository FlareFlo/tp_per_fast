mod reader_thread;
mod geo_to_bezirk;

use std::{
	fs,
	net::{Ipv6Addr, SocketAddrV6, TcpListener},
	os::fd::AsFd,
	sync::Arc,
	thread::sleep,
	time::Duration,
};

use geo::Geometry;
use nix::sys::{socket, socket::sockopt::ReusePort};
use prost::Message;
use wkt::TryFromWkt;
use geo_to_bezirk::binary_search::BinarySearch;
use crate::{
	protobufs::File,
	reader_thread::reader_thread,
};

pub mod protobufs {
	pub use Bezirk as ProtobufBezirk;
	include!(concat!(env!("OUT_DIR"), "/geodata.rs"));
	include!(concat!(env!("OUT_DIR"), "/wire.rs"));
}

pub struct BezirkeData {
	pub data:           Vec<Bezirk>,
}

#[derive(Clone, Debug)]
pub struct Bezirk {
	pub identifier: u64,
	pub parents:    Vec<u64>,
	pub name:       String,
	pub location:   Geometry,
}

impl BezirkeData {
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
			data:           parsed,
		}
	}
}

fn main() {
	let bezirke = protobufs::File::decode(
		fs::read("/home/flareflo/tp_per/group-b/geodata/build/bezirke-12.geodata")
			.unwrap()
			.as_slice(),
	)
	.unwrap();
	let bezirke = BezirkeData::new(bezirke);
	let lut = Arc::new(BinarySearch::new_with_defaults(10, bezirke));

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
