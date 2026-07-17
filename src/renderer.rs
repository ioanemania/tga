use crate::{set_pixel::{Color, SetPixel}, vector::Vector2I};

pub fn render_line(target: &mut impl SetPixel, start: Vector2I, end: Vector2I, color: Color) {
    target.set_pixel(start, color).unwrap();
    target.set_pixel(end, color).unwrap();
}
