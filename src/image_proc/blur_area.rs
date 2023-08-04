use image::DynamicImage;

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
            image: image,
            blur: blur,
            crop_position: crop_position,
            crop_size: crop_size
        }
    }
    pub fn get_crop_data(&self) -> ((u32,u32),(u32,u32)) {
        (self.crop_position,self.crop_size)
    }
    pub fn show(&self) -> DynamicImage {
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