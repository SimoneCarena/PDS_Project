use image::Rgba;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Color{
    pub color: Rgba<u8>
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        let alpha = alpha*(255 as f32);
        Self { 
            color: Rgba([red, green, blue, alpha as u8])
        }
    }
}

pub fn convert_f32_u8(rgb: [f32;3]) -> [u8;3]{
    [(rgb[0] * 255.0) as u8, (rgb[1] * 255.0) as u8, (rgb[2] * 255.0) as u8]
}

pub fn convert_u8_f32(rgb: [u8;3]) -> [f32;3]{
    [(rgb[0] as f32)/255.0, (rgb[1] as f32)/255.0, (rgb[2] as f32)/255.0]
}
