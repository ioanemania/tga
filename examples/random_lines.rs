use std::fs::File;

use rand::random_range;
use tga::{ImageBits, ImageKind, TGAImage, Vector2I, renderer, set_pixel::Color};

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

fn main() {
    let mut image = TGAImage::new(WIDTH as u16, HEIGHT as u16, ImageKind::RGB, ImageBits::N24);

    for _ in 0..(1 << 24) {
            renderer::render_line(
                &mut image,
                Vector2I::new(
                    random_range(0..WIDTH) as i16,
                    random_range(0..HEIGHT) as i16,
                ),
                Vector2I::new(
                    random_range(0..WIDTH) as i16,
                    random_range(0..HEIGHT) as i16,
                ),
                Color::Rgb24(
                    random_range(0..=255),
                    random_range(0..=255),
                    random_range(0..=255),
                ),
            )
    }

    let mut file = File::options()
        .create(true)
        .write(true)
        .open("random_lines.tga")
        .unwrap();

    image.write(&mut file).unwrap();
}
