use std::{env, fs::File, process};

use tga::{TGAImage, Vector2I, wavefront_obj::Model};

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: render_obj <file.obj>");
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

    let mut image = TGAImage::new(
        WIDTH as u16,
        HEIGHT as u16,
        tga::ImageKind::RGB,
        tga::ImageBits::N24,
    );

    for face in model.faces {
        let v1 = Vector2I::new(
            ((model.vertices[face.vertex_indices[0] - 1].x + 1.) * (WIDTH / 3) as f32) as i16,
            ((model.vertices[face.vertex_indices[0] - 1].y + 1.) * (HEIGHT / 3) as f32) as i16,
        );
        let v2 = Vector2I::new(
            ((model.vertices[face.vertex_indices[1] - 1].x + 1.) * (WIDTH / 3) as f32) as i16,
            ((model.vertices[face.vertex_indices[1] - 1].y + 1.) * (HEIGHT / 3) as f32) as i16,
        );
        let v3 = Vector2I::new(
            ((model.vertices[face.vertex_indices[2] - 1].x + 1.) * (WIDTH / 3) as f32) as i16,
            ((model.vertices[face.vertex_indices[2] - 1].y + 1.) * (HEIGHT / 3) as f32) as i16,
        );

        dbg!(v1, v2, v3);

        image
            .draw_line(v1, v2, tga::Color::Rgb24(255, 0, 0))
            .unwrap_or_default();
        image
            .draw_line(v1, v3, tga::Color::Rgb24(255, 0, 0))
            .unwrap_or_default();
        image
            .draw_line(v2, v3, tga::Color::Rgb24(255, 0, 0))
            .unwrap_or_default();
    }

    let mut file = File::options()
        .create(true)
        .write(true)
        .open("obj.tga")
        .unwrap();

    image.write(&mut file).unwrap();
}
