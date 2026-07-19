use std::fs::File;

use tga::{ImageBits, ImageKind, TGAImage, Vector2I, set_pixel::SetPixel};

const WIDTH: usize = 200;
const HEIGHT: usize = 200;

fn main() {
    let mut image = TGAImage::new(WIDTH as u16, HEIGHT as u16, ImageKind::RGB, ImageBits::N24);

    image.set_pixel(Vector2I::new(100, 100), tga::set_pixel::Color::Rgb24(255, 255, 255)).unwrap();

    let mut file = File::options()
        .create(true)
        .write(true)
        .open("black_image.tga")
        .unwrap();

    image.write(&mut file).unwrap();
}
