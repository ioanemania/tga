use std::fs::File;

use tga::{ImageBits, ImageKind, TGAImage};

fn main() {
    let mut image = TGAImage::new(64, 64, ImageKind::RGB, ImageBits::N24);

    image.set(32, 32, tga::Color::Rgb24(0, 255, 0)).unwrap();

    let mut file = File::options()
        .create(true)
        .write(true)
        .open("black_image.tga").unwrap();

    image.write(&mut file).unwrap();
}
