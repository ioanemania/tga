use crate::{
    Vector2I,
    set_pixel::{Color, SetPixel},
};

pub fn render_line(target: &mut impl SetPixel, start: Vector2I, end: Vector2I, color: Color) {
    if (end.y - start.y).abs() < (end.x - start.x).abs() {
        if start.x > end.x {
            render_line_low(target, end, start, color);
        } else {
            render_line_low(target, start, end, color);
        }
    } else {
        if start.y > end.y {
            render_line_high(target, end, start, color);
        } else {
            render_line_high(target, start, end, color);
        }
    }
}

fn render_line_low(target: &mut impl SetPixel, start: Vector2I, end: Vector2I, color: Color) {
    let dx = end.x - start.x;
    let mut dy = end.y - start.y;
    let mut yi = 1;

    if dy < 0 {
        dy = -dy;
        yi = -yi;
    }

    let mut diff = (2 * dy) - dx;
    let mut y = start.y;

    for x in start.x..=end.x {
        target.set_pixel(Vector2I::new(x, y), color).unwrap();

        if diff > 0 {
            y = y + yi;
            diff = diff + (2*(dy - dx))
        } else {
            diff = diff + 2*dy;
        }
    }
}

fn render_line_high(target: &mut impl SetPixel, start: Vector2I, end: Vector2I, color: Color) {
    let mut dx = end.x - start.x;
    let dy = end.y - start.y;
    let mut xi = 1;

    if dx < 0 {
        dx = -dx;
        xi = -xi;
    }

    let mut diff = (2 * dx) - dy;
    let mut x = start.x;

    for y in start.y..=end.y {
        target.set_pixel(Vector2I::new(x, y), color).unwrap();

        if diff > 0 {
            x = x + xi;
            diff = diff + (2*(dx - dy))
        } else {
            diff = diff + 2*dx;
        }
    }
}
