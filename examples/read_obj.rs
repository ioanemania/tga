use std::{env, fs::File, process};

use tga::wavefront_obj::Model;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: read_obj <file.obj>");
        process::exit(1);
    });

    let mut file = File::open(&path).unwrap_or_else(|e| {
        eprintln!("Error opening '{}': {}", path, e);
        process::exit(1);
    });

    let model = Model::from_reader(&mut file).unwrap_or_else(|e| {
        eprintln!("Error reading model from '{}': {}", path, e);
        process::exit(1);
    });

    println!("Model '{}':", path);
    println!("  vertices: {}", model.vertices.len());
    println!("  faces:    {}", model.faces.len());

    for (i, v) in model.vertices.iter().enumerate() {
        println!("  v{}: {} {} {} {}", i + 1, v.x, v.y, v.z, v.w);
    }

    for (i, f) in model.faces.iter().enumerate() {
        let indices: Vec<String> = f.vertex_indices.iter().map(|i| i.to_string()).collect();
        println!("  f{}: {}", i + 1, indices.join(" "));
    }
}
