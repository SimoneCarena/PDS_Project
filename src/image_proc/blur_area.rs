use image::DynamicImage;
use imageproc::rect::Rect;

use super::colors::Color;

#[derive(Debug, Clone)]
pub struct BlurArea{
    image: DynamicImage,
    blur: DynamicImage,
    crop_position: (u32, u32),
    crop_size: (u32, u32)
}

impl BlurArea {
    pub fn new(image: DynamicImage, blur: DynamicImage, crop_position: (u32, u32), crop_size: (u32, u32)) -> Self {
        Self {
            image,
            blur,
            crop_position,
            crop_size
        }
    }
    pub fn get_crop_data(&self) -> ((u32,u32),(u32,u32)) {
        (self.crop_position,self.crop_size)
    }
    pub fn show(&self) -> DynamicImage {

        let borders = crate::load_assets::load_borders().unwrap();
        let (x,y) = self.crop_position;
        let x = x as i64;
        let y = y as i64;
        let (w,h) = self.crop_size;
        let w = w as i64;
        let h = h as i64;
        let corner_size = u32::min(self.blur.width()/20,self.blur.height()/20) as i64;

        let upleft_border_image = borders.get("tl_corner").unwrap().clone().resize(corner_size as u32, corner_size as u32, image::imageops::FilterType::Lanczos3);
        let upright_border_image = borders.get("tr_corner").unwrap().clone().resize(corner_size as u32, corner_size as u32, image::imageops::FilterType::Lanczos3);
        let bottomleft_border_image = borders.get("bl_corner").unwrap().clone().resize(corner_size as u32, corner_size as u32, image::imageops::FilterType::Lanczos3);
        let bottomright_border_image = borders.get("br_corner").unwrap().clone().resize(corner_size as u32, corner_size as u32, image::imageops::FilterType::Lanczos3);

        let mut blur = self.blur.clone();
        let image = self.image.clone().crop(self.crop_position.0, self.crop_position.1, self.crop_size.0, self.crop_size.1);
        image::imageops::overlay(&mut blur, &image, self.crop_position.0.into(), self.crop_position.1.into());
        image::imageops::overlay(&mut blur, &upleft_border_image, x, y);
        image::imageops::overlay(&mut blur, &upright_border_image, x+w-corner_size, y);
        image::imageops::overlay(&mut blur, &bottomleft_border_image, x, y+h-corner_size);
        image::imageops::overlay(&mut blur, &bottomright_border_image, x+w-corner_size, y+h-corner_size);

        let rect = Rect::at(x as i32, y as i32).of_size(w as u32, h as u32);
        imageproc::drawing::draw_hollow_rect_mut(&mut blur, rect, Color::new(255, 255, 255, 1.0).color);

        blur
    }
    pub fn save(&self) -> DynamicImage {
        let mut blur = self.blur.clone();
        let image = self.image.clone().crop(self.crop_position.0, self.crop_position.1, self.crop_size.0, self.crop_size.1);
        image::imageops::overlay(&mut blur, &image, self.crop_position.0.into(), self.crop_position.1.into());
        blur
    }
    pub fn resize(&mut self, new_crop_position: (u32, u32), new_crop_size: (u32, u32)) {
        self.crop_position = new_crop_position;
        self.crop_size = new_crop_size;
    }
}