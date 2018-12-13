extern crate protoc_rust;
extern crate walkdir;

use walkdir::WalkDir;
use protoc_rust::Customize;

use std::path::{Path, PathBuf};

/// Finds all .proto files in `path` and subfolders and returns a vector of their paths.
fn get_proto_files<P: AsRef<Path>>(path: &P) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| {
            let e = e.ok()?;
            if e.path().extension()?.to_str() == Some("proto") {
                Some(e.path().to_str().map(|s| s.replace("\\","/")).unwrap())
            } else {
                None
            }
        }).collect()
}

fn main() {
    println!("cargo:rerun-if-changed={}", "src/lib_types/proto/exonum");
    println!("cargo:rerun-if-changed={}", "src/user_types/proto");
    //Here we generate library types from .proto

    let proto_files = get_proto_files(&Path::new("src/lib_types/proto"));
    let p_vec = proto_files.iter()
            .map(|s| s.as_str()).collect::<Vec<_>>();
    eprintln!("{:?}",p_vec);
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/lib_types/proto",
        input: &p_vec,
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
