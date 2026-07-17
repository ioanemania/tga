use crate::{Vector2I, set_pixel::{Color, SetPixel}};

pub fn render_line(target: &mut impl SetPixel, start: Vector2I, end: Vector2I, color: Color) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let step = std::cmp::max(dx.abs(), dy.abs());

    if step == 0 {
        target.set_pixel(start, color).unwrap();
        return;
    }

    let step_x: f32 = dx as f32 / step as f32;
    let step_y: f32 = dy as f32 / step as f32;

    for i in 0..=step {
        let point = Vector2I::new(start.x + (step_x * i as f32) as i16, start.y + (step_y * i as f32) as i16);
        target.set_pixel(point, color).unwrap();
    }
}
