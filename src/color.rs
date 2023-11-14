use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub fn from_bytes(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r, g, b, a
        }
    }

    pub fn from_hex(rgba: u32) -> Self {
        let bytes = rgba.to_be_bytes();
        Self {
            r: bytes[0], g: bytes[1], b: bytes[2], a: bytes[3]
        }
    }

    pub fn into_argb(&self) -> u32 {
        u32::from_be_bytes([self.a, self.r, self.g, self.b])
    }

    pub fn mix_three(a: Color, b: Color, c: Color, mask: Vec3) -> Color {
        let mut color = Color::default();
        color.r = ((a.r as f32 * mask.x + b.r as f32 * mask.y + c.r as f32 * mask.z)) as u8;
        color.b = ((a.b as f32 * mask.x + b.b as f32 * mask.y + c.b as f32 * mask.z)) as u8;
        color.g = ((a.g as f32 * mask.x + b.g as f32 * mask.y + c.g as f32 * mask.z)) as u8;
        color
    }

    pub fn red() -> Self {
        Self { r: 255, g: 0, b: 0, a: 255 }
    }
    pub fn green() -> Self {
        Self { r: 0, g: 255, b: 0, a: 255 }
    }
    pub fn blue() -> Self {
        Self { r: 0, g: 0, b: 255, a: 255 }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 0 }
    }
}
