use image::DynamicImage;

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
}