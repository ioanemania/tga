use crate::Vector2I;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    /// 8-bit greyscale or indexed
    L8(u8),
    /// 16-bit packed (1-5-5-5: attribute, R, G, B)
    Rgb16(u16),
    /// 24-bit (R, G, B) — stored as B, G, R in TGA
    Rgb24(u8, u8, u8),
    /// 32-bit (R, G, B, A) — stored as B, G, R, A in TGA
    Rgba32(u8, u8, u8, u8),
}

pub trait SetPixel {
    fn set_pixel(&mut self, point: Vector2I, color: Color) -> Result<(), &'static str>;
}
