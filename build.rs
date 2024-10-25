use std::env;
use std::io::Result;
fn main() -> Result<()> {
	dotenv::from_path(".env").unwrap();
	let p = env::var("PROT_DEF").unwrap();

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
