extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    //Here we generate library types from .proto
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/lib_types/proto",
        input: &["src/lib_types/proto/hash.proto"],
        includes: &["src/lib_types/proto"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");

    //Here we generate user types from .proto
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/user_types/proto",
        input: &["src/user_types/proto/mymsg.proto"],
        includes: &["src/lib_types/proto", "src/user_types/proto"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");
}
