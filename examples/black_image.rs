use std::fs::File;

use tga::{ImageBits, ImageKind, TGAImage, Vector2I, renderer};

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

fn main() {
    let mut image = TGAImage::new(WIDTH as u16, HEIGHT as u16, ImageKind::RGB, ImageBits::N24);

    renderer::render_line(
        &mut image,
        Vector2I::new(10, 40),
        Vector2I::new(19, 36),
        tga::set_pixel::Color::Rgb24(255, 0, 0),
    );

    // Horizontal Line
    renderer::render_line(
        &mut image,
        Vector2I::new(2, 2),
        Vector2I::new(10, 2),
        tga::set_pixel::Color::Rgb24(255, 0, 0),
    );

    // Vertical Line
    renderer::render_line(
        &mut image,
        Vector2I::new(2, 10),
        Vector2I::new(3, 2),
        tga::set_pixel::Color::Rgb24(0, 255, 0),
    );

    let mut file = File::options()
        .create(true)
        .write(true)
        .open("black_image.tga")
        .unwrap();

    image.write(&mut file).unwrap();
}
