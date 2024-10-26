use std::env;
use std::io::Result;
fn main() -> Result<()> {
	let p = if env::var("IN_NIX").is_ok() {
		"./geodata".to_owned()
	} else {
		dotenv::from_path(".env").unwrap();
		env::var("GEO_PATH").unwrap()
	}  + "/proto";


	let includes: &[&str] = &[&p];
	prost_build::compile_protos(
		&[
			format!("{p}/geodata.proto"),
			format!("{p}/message.proto"),
		],
		includes,
	)?;
	Ok(())
}
