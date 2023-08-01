use image::Rgba;

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