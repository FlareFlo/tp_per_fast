use dotenv::dotenv;
use std::{env, fs};
use prost::Message;

include!(concat!(env!("OUT_DIR"), "/geodata.rs"));
include!(concat!(env!("OUT_DIR"), "/wire.rs"));

pub fn from_env() -> File {
	let p = if env::var("IN_NIX").is_ok() {
		"./geodata".to_owned()
	} else {
		dotenv().unwrap();
		env::var("GEO_PATH").unwrap()
	} + "/build/bezirke-12.geodata";

	let bezirke = File::decode(
		fs::read(p)
			.unwrap()
			.as_slice(),
	)
	.unwrap();
	bezirke
}