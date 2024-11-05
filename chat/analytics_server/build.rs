use std::{fs, process::Command};

use anyhow::Result;

// prost 只能编译proto的message，还不能编译service
// 需要使用tonic，tonic-build编译service

fn main() -> Result<()> {
    let path = "src/pb";
    fs::create_dir_all(path)?;
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["../../protos/messages.proto"], &["../../protos"])?;

    println!("cargo:rerun-if-changed=../../protos/messages.proto");
    // run format
    let status = Command::new("cargo")
        .arg("fmt")
        .status()
        .expect("failed to format code");
    assert!(status.success());
    Ok(())
}
