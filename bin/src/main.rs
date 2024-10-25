mod geo_to_bezirk;
mod reader_thread;
mod stattrack;

use std::{net::{Ipv6Addr, SocketAddrV6, TcpListener}, os::fd::AsFd, sync::Arc, thread::sleep, time::Duration};
use geo::Geometry;
use nix::sys::{socket, socket::sockopt::ReusePort};
use wkt::TryFromWkt;
use protodefs::{from_env, File};
use crate::{geo_to_bezirk::rtree::RStarTree, reader_thread::reader_thread};

pub struct BezirkeData {
	pub data: Vec<Bezirk>,
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

		Self { data: parsed }
	}
}

fn main() {
	let bezirke = from_env();
	let bezirke = BezirkeData::new(bezirke);
	let lut = Arc::new(RStarTree::new(bezirke.data));
	stattrack::spawn_stattrack();

	let port = 1234;
	let addr = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);
	let server = TcpListener::bind(addr).unwrap();
	socket::setsockopt(&server.as_fd(), ReusePort, &true).unwrap();

	for stream in server.incoming() {
		match stream {
			Ok(socket) => {
				reader_thread(socket, lut.clone());
			},
			Err(e) => println!("couldn't get client: {e:?}"),
		}
	}
	sleep(Duration::from_secs(100))
}
