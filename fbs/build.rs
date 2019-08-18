extern crate flatc_rust;

use std::path::Path;

fn main() {
    flatc_rust::run(flatc_rust::Args {
        inputs: &[Path::new("./p2p.fbs")],
        out_dir: Path::new("./src/"),
        ..Default::default()
    }).expect("flatc");
}