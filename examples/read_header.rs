use std::{env, fs::File, process};

use tga::TGAHeader;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: read_header <file.tga>");
        process::exit(1);
    });

    let mut file = File::open(&path).unwrap_or_else(|e| {
        eprintln!("Error opening '{}': {}", path, e);
        process::exit(1);
    });

    let header = TGAHeader::read(&mut file).unwrap_or_else(|e| {
        eprintln!("Error reading header from '{}': {}", path, e);
        process::exit(1);
    });

    println!("TGA Header: {:#?}", header);
}
