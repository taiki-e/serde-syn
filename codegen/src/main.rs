// Based on https://github.com/dtolnay/syn/tree/1.0.5/codegen.
//
// This crate generates the Syn trait in syn-serde programmatically from
// the syntax tree description.

#![warn(rust_2018_idioms, single_use_lifetimes, unreachable_pub)]

const SYN_JSON: &str = "../syn.json";

mod ast_enum;
mod convert;
mod file;
mod gen;

use std::fs;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let defs = fs::read_to_string(file::manifest_dir().join(SYN_JSON))?;
    let defs = serde_json::from_str(&defs)?;

    ast_enum::generate(&defs)?;
    convert::generate(&defs)?;
    Ok(())
}
