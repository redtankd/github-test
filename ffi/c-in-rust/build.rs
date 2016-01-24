
extern crate cmake;

// use cmake;

fn main() {
	let dst = cmake::build("libexample");

	println!("cargo:rustc-link-search=native={}/lib", dst.display());
	println!("cargo:rustc-link-lib=static=example");
}