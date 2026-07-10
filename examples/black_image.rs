use std::fs::File;

use tga::{ImageBits, ImageKind, TGAImage};

fn main() {
    let image = TGAImage::new(64, 64, ImageKind::RGB, ImageBits::N24);

    let mut file = File::options()
        .create(true)
        .write(true)
        .open("black_image.tga").unwrap();

    image.write(&mut file).unwrap();
}
