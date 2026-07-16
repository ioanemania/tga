use std::fs::File;

use rand::random_range;
use tga::{Color, ImageBits, ImageKind, TGAImage, Vector2I};

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

fn main() {
    let mut image = TGAImage::new(WIDTH as u16, HEIGHT as u16, ImageKind::RGB, ImageBits::N24);

    for _ in 0..(1<<24) {
        image.draw_line(
            Vector2I { x: random_range(0..WIDTH) as i16, y: random_range(0..HEIGHT) as i16},
            Vector2I { x: random_range(0..WIDTH) as i16, y: random_range(0..HEIGHT) as i16},
            Color::Rgb24(random_range(0..=255), random_range(0..=255), random_range(0..=255)),
        ).unwrap();
    }

    let mut file = File::options()
        .create(true)
        .write(true)
        .open("random_lines.tga").unwrap();

    image.write(&mut file).unwrap();
}

