use crate::{set_pixel::{Color, SetPixel}, vector::Vector2I};

pub fn render_line(target: &mut impl SetPixel, start: Vector2I, end: Vector2I, color: Color) {
    let slope: f32 = ((end.y - start.y) as f32 / (end.x - start.x) as f32).abs();

    for x in 0..=(end.x - start.x) {
        let point = Vector2I{ x: x + start.x, y: start.y - (slope * x as f32) as i16 };
        dbg!(slope, point);
        target.set_pixel(point, color).unwrap();
    }
}
