use image::{DynamicImage, imageops::overlay};

#[derive(Clone)]
pub struct Layer {
    pub layer: DynamicImage,
    pub layer_type: LayerType
}

#[derive(Clone)]
pub enum LayerType {
    Text,
    Shape,
    FreeHandDrawing
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
}