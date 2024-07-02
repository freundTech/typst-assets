use std::{env, io};
use std::fs::File;
use std::path::Path;

use flate2::Compression;
use flate2::read::GzEncoder;

fn main() -> Result<(), io::Error>{
    let out_dir = env::var("OUT_DIR").unwrap();
    let tar_gz = File::create(Path::new(&out_dir).join("fonts.tar.gz"))?;
    let enc = GzEncoder::new(tar_gz, Compression::best());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("", "files/fonts").unwrap();

    print!("cargo:rerun-if-changed=files/fonts");
    Ok(())
}