#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[package]
name = "clap_test"
version = "0.2.0"
edition = "2021"
[dependencies]
clap = { version = "4.5", features = ["derive"] }
---

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, help = "Path to config")]
    config: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}