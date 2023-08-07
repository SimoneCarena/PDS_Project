use image::{DynamicImage, imageops::overlay, RgbaImage};

use super::colors::Color;

#[derive(Clone)]
pub struct Layer {
    pub layer: DynamicImage,
    pub layer_type: LayerType
}

#[derive(Clone, PartialEq, Eq)]
pub enum LayerType {
    Text,
    //hold the upper-left corner and the size
    Shape(((u32,u32),(u32,u32))),
    FreeHandDrawing,
    BaseImage
}

impl Layer {
    pub fn new(layer: DynamicImage, layer_type: LayerType) -> Self {
        Self {
            layer: layer,
            layer_type: layer_type
        }
    }
    pub fn show(&self) -> DynamicImage{
        self.layer.clone()
    }

    pub fn show_rubber(&self, base: &Layer) -> DynamicImage {
        let mut image = base.layer.clone();
        overlay(&mut image, &self.layer, 0, 0);
        image
    }

    pub fn show_higlight(&self, base: &Layer) -> DynamicImage {
        let mut image = base.layer.clone();
        overlay(&mut image, &self.layer, 0, 0);
        image
    }

    pub fn show_shape(&self, base: &Layer) -> DynamicImage {
        let mut image = base.layer.clone();
        let mut area = RgbaImage::new(image.width(), image.height());
        let (pos, size) = self.get_pos_size().unwrap();
        let rect = imageproc::rect::Rect::at(pos.0 as i32, pos.1 as i32).of_size(size.0, size.1);
        let blue = Color::new(0, 255, 255, 1.0);

        let width = self.layer.width();
        let height = self.layer.height();
        println!("{:?},{:?}",(size.0,size.1),pos);

        let radius = (u32::min(width,height)/100) as i32;

        imageproc::drawing::draw_hollow_rect_mut(&mut area, rect, blue.color);
        imageproc::drawing::draw_filled_circle_mut(&mut area, (pos.0 as i32, pos.1 as i32), radius, blue.color);
        imageproc::drawing::draw_filled_circle_mut(&mut area, ((pos.0+size.0) as i32, pos.1 as i32), radius, blue.color);
        imageproc::drawing::draw_filled_circle_mut(&mut area, (pos.0 as i32, (pos.1+size.1) as i32), radius, blue.color);
        imageproc::drawing::draw_filled_circle_mut(&mut area, ((pos.0+size.0) as i32, (pos.1+size.1) as i32), radius, blue.color);

        overlay(&mut image, &self.layer, 0, 0);
        overlay(&mut image, &area, 0, 0);

        image
    }

    pub fn draw_shape(&self, base: &Layer) -> DynamicImage {
        let mut image = base.layer.clone();
        overlay(&mut image, &self.layer, 0, 0);
        image
    }

    pub fn get_pos_size(&self) -> Option<((u32,u32),(u32,u32))> {
        match self.layer_type {
            LayerType::Shape((pos,size)) => {
                Some((pos,size))
            },
            LayerType::Text => None,
            LayerType::FreeHandDrawing => None,
            LayerType::BaseImage => None
        }
    }
}