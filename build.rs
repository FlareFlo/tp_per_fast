use std::io::Result;
fn main() -> Result<()> {
	let includes: &[&'static str] = &["/home/flareflo/tp_per/group-b/geodata/proto"];
	prost_build::compile_protos(&["/home/flareflo/tp_per/group-b/geodata/proto/geodata.proto", "/home/flareflo/tp_per/group-b/geodata/proto/message.proto"], includes)?;
	Ok(())
}