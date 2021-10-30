use protoc_rust;

fn main() {
    protoc_rust::Codegen::new()
        .inputs(&["src/proto/data.proto"])
        .out_dir("src/proto")
        .run()
        .expect("Running protoc failed.");
}