use std::io::{Read, Write};

use bitvec::vec::BitVec;
use bytemuck::{Pod, Zeroable, bytes_of, checked::from_bytes};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2I {
    pub x: i16,
    pub y: i16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ColourMapKind {
    None = 0,
    Has = 1,
}

unsafe impl Zeroable for ColourMapKind {}
unsafe impl Pod for ColourMapKind {}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ImageKind {
    None = 0,
    Indexed = 1,
    RGB = 2,
    Grey = 3,
}

unsafe impl Zeroable for ImageKind {}
unsafe impl Pod for ImageKind {}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ImageBits {
    N8 = 8,
    N16 = 16,
    N24 = 24,
    N32 = 32,
}

unsafe impl Zeroable for ImageBits {}
unsafe impl Pod for ImageBits {}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum ColourMapBits {
    None = 0,
    N8 = 8,
    N15 = 15,
    N16 = 16,
    N24 = 24,
    N32 = 32,
}

unsafe impl Zeroable for ColourMapBits {}
unsafe impl Pod for ColourMapBits {}

#[derive(Copy, Clone, Debug, Pod, Zeroable)]
#[repr(C)]
pub struct TGADescriptor(pub u8);

impl TGADescriptor {}

#[derive(Copy, Clone, Debug, Pod, Zeroable)]
#[repr(C, packed)]
pub struct TGAHeader {
    /// Size of ID field that follows 18 byte header (0 usually)
    pub identity_size: u8,

    pub colour_map_kind: ColourMapKind,

    pub image_kind: ImageKind,

    /// First colour map entry in palette
    pub colour_map_start: u16,

    /// Number of colours in palette
    pub colour_map_length: u16,

    /// Number of bits per palette entry
    pub colour_map_bits: ColourMapBits,

    /// Image x origin
    pub x_start: u16,

    /// Image y origin
    pub y_start: u16,

    /// Image width
    pub width: u16,

    /// Image height
    pub height: u16,

    /// Image bits per pixel
    pub bits: ImageBits,

    /// Image descriptor bits
    pub descriptor: TGADescriptor,
}

impl TGAHeader {
    pub fn read<R: Read>(read: &mut R) -> std::io::Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        read.read_exact(&mut buffer)?;

        Ok(*from_bytes(&buffer))
    }
}

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

impl Color {
    pub fn image_bits(&self) -> ImageBits {
        match self {
            Color::L8(_) => ImageBits::N8,
            Color::Rgb16(_) => ImageBits::N16,
            Color::Rgb24(..) => ImageBits::N24,
            Color::Rgba32(..) => ImageBits::N32,
        }
    }

    /// Serialise to bytes in TGA byte order (BGR / BGRA).
    fn to_tga_bytes(self) -> [u8; 4] {
        match self {
            Color::L8(v) => [v, 0, 0, 0],
            Color::Rgb16(v) => {
                let [lo, hi] = v.to_le_bytes();
                [lo, hi, 0, 0]
            }
            Color::Rgb24(r, g, b) => [b, g, r, 0],
            Color::Rgba32(r, g, b, a) => [b, g, r, a],
        }
    }
}

pub struct TGAImage {
    header: TGAHeader,
    data: BitVec<u8>,
}

impl TGAImage {
    pub fn new(width: u16, height: u16, kind: ImageKind, bits: ImageBits) -> Self {
        let total_bits = width as usize * height as usize * bits as usize;
        let data = BitVec::repeat(false, total_bits);

        Self {
            header: TGAHeader {
                identity_size: 0,
                colour_map_kind: ColourMapKind::None,
                image_kind: kind,
                colour_map_start: 0,
                colour_map_length: 0,
                colour_map_bits: ColourMapBits::None,
                x_start: 0,
                y_start: 0,
                width,
                height,
                bits,
                descriptor: TGADescriptor(0b00001111),
            },
            data,
        }
    }

    pub fn write<W: Write>(&self, write: &mut W) -> std::io::Result<()> {
        let header_bytes: &[u8] = bytes_of(&self.header);
        write.write_all(header_bytes)?;

        let data_bytes: &[u8] = self.data.as_raw_slice();
        write.write_all(data_bytes)?;

        Ok(())
    }

    pub fn set(&mut self, point: Vector2I, color: Color) -> Result<(), &'static str> {
        if color.image_bits() != self.header.bits {
            return Err("color bit depth does not match image bit depth");
        }

        if point.x >= self.header.width as i16 || point.y >= self.header.height as i16 {
            return Err("pixel coordinates out of bounds");
        }

        let bpp = self.header.bits as usize;
        let pixel_bit_offset = ((
            (self.header.width as usize - point.y as usize) * self.header.width as usize)
            + point.x as usize)
            * bpp;

        let bytes = color.to_tga_bytes();
        let byte_count = bpp / 8;

        for i in 0..byte_count {
            let byte = bytes[i];
            let bit_offset = pixel_bit_offset + i * 8;
            for bit in 0..8usize {
                // TGA is little-endian: LSB first within each byte.
                self.data.set(bit_offset + bit, (byte >> bit) & 1 == 1);
            }
        }

        Ok(())
    }
}
