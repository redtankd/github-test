extern crate protoc_rust;

fn main() {
	protoc_rust::run(protoc_rust::Args {
        out_dir: "src/proto",
        input: &["src/proto/data.proto"],
        ..Default::default()
    }).expect("protoc");
}