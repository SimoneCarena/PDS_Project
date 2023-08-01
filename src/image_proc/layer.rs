use image::DynamicImage;

pub struct Layer {
    pub layer: DynamicImage,
    pub layer_type: LayerType
}

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
}