use std::{
	io::{Read, Write},
	net::{SocketAddr, TcpStream},
	sync::Arc,
	thread::{sleep, spawn},
	time::{Duration, Instant},
};

use prost::Message;

use crate::{
	protobufs::{
		client_message::Request,
		client_response::{Code, Response},
		LocationUpdateResponse,
	},
	BezirkLUT,
};

pub fn reader_thread(mut socket: TcpStream, lut: Arc<BezirkLUT>) {
	let _handle = spawn(move || {
		loop {
			let mut response = crate::protobufs::ClientResponse::default();
			let len = &mut [0; 4];
			socket.read_exact(len).unwrap();
			let parsed_len = u32::from_be_bytes(*len);

			let mut res = vec![0; parsed_len as usize];
			socket.read_exact(&mut res).unwrap();
			let request = crate::protobufs::ClientMessage::decode(res.as_slice()).unwrap();
			response.request_id = request.request_id;
			response.set_code(Code::Ok);

			if let Some(Request::Location(location_request)) = request.request {
				let start = Instant::now();
				let res = lut.binary_lookup(location_request.latitude, location_request.longitude);
				// dbg!(start.elapsed(), res);
				response.response = Some(Response::Location(LocationUpdateResponse {
					car_id:   location_request.car_id,
					location: res.unwrap_or_default().to_string(),
				}))
			}
			let res_parsed = response.encode_to_vec();
			socket
				.write_all(&(res_parsed.len() as u32).to_be_bytes())
				.unwrap();
			socket
				.write_all(response.encode_to_vec().as_slice())
				.unwrap();
		}
	});
}
