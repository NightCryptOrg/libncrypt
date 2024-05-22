use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
	// Call cbindgen to create FFI header
	println!("cargo:rerun-if-changed=cbindgen.toml");
	const C_HEADER_FILE: &str = "include/ncrypt.h";
	let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
	cbindgen::generate(crate_dir)
		.or_else(|err| {
			eprintln!("Failed to generate C header {}", C_HEADER_FILE);
			Err(err)
		})?
		.write_to_file(C_HEADER_FILE);

	Ok(())
}
