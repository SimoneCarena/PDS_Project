use image::{DynamicImage, imageops::overlay, RgbaImage};

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