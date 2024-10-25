use protodefs::LocationUpdateResponse;
use protodefs::client_response::Response;
use protodefs::client_message::Request;
use protodefs::client_response::Code;
use std::{
	io::{Read, Write},
	net::TcpStream,
	sync::Arc,
	thread::spawn,
};

use prost::Message;

use crate::{
	geo_to_bezirk::GeoToBezirk,
};

pub fn reader_thread(mut socket: TcpStream, lut: Arc<impl GeoToBezirk + Sync + Send + 'static>) {
	let _handle = spawn(move || loop {
		let mut response = protodefs::ClientResponse::default();
		let len = &mut [0; 4];
		socket.read_exact(len).unwrap();
		let parsed_len = u32::from_be_bytes(*len);

		let mut res = vec![0; parsed_len as usize];
		socket.read_exact(&mut res).unwrap();
		let request = protodefs::ClientMessage::decode(res.as_slice()).unwrap();
		response.request_id = request.request_id;
		response.set_code(Code::Ok);

		if let Some(Request::Location(location_request)) = request.request {
			let res = lut.lookup(location_request.latitude, location_request.longitude);
			response.response = Some(Response::Location(LocationUpdateResponse {
				car_id:   location_request.car_id,
				location: res.map(|e| e.name.clone()).unwrap_or_default(),
			}))
		}
		let res_parsed = response.encode_to_vec();
		socket
			.write_all(&(res_parsed.len() as u32).to_be_bytes())
			.unwrap();
		socket
			.write_all(response.encode_to_vec().as_slice())
			.unwrap();
	});
}
