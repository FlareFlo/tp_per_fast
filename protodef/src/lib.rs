use dotenv::dotenv;
use std::{env, fs};
use prost::Message;

include!(concat!(env!("OUT_DIR"), "/geodata.rs"));
include!(concat!(env!("OUT_DIR"), "/wire.rs"));

pub fn from_env() -> File {
	dotenv().unwrap();
	let p = env::var("GEO_PATH").unwrap();
	let bezirke = File::decode(
		fs::read(p)
			.unwrap()
			.as_slice(),
	)
	.unwrap();
	bezirke
}