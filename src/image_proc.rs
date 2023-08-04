pub mod extensions;
pub mod polygon;
pub mod colors;
pub mod image_errors;
pub mod layer;
pub mod blur_area;

use image::{DynamicImage, RgbaImage};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::fs::File;
use extensions::Extensions;
use imageproc::drawing;
use polygon::Polygon;
use colors::Color;
use image_errors::ImageManipulationError;
use layer::{Layer, LayerType};
use blur_area::BlurArea;
use std::borrow::Cow;

///Incremental counter for files whose name is not specified when saved
static UNNAMED_COUNTER: AtomicUsize = AtomicUsize::new(0);

///Structure containing the base screenshot, its size and the additional layers of editing applied to it
pub struct Image {
    base: DynamicImage,
    layers: VecDeque<Layer>,
    size: (u32, u32)
}

impl Image {
    ///Returns an Image structure, wrapped in a Result, given the path where to retrieve it from. 
    ///In case of failure an ImageManipulationError is returned, with the IOError variant
    pub fn open(path: &str) -> Result<Self,ImageManipulationError> {
        let image = image::open(path)?;
        let width = image.width();
        let height = image.height();
        Ok(
            Self {
                base: image,
                layers: VecDeque::new(),
                size: (width, height)
            }
        )
    }
    ///Creates an Image from an existing DynamicImage
    pub fn from_image(image: DynamicImage) -> Self {
        let width = image.width();
        let height = image.height();
        Self {
            base: image,
            layers: VecDeque::new(),
            size: (width, height)
        }
    }
    ///Return the width of the image
    pub fn width(&self) -> u32 {
        return self.base.width();
    }
    ///Returns the height of the image
    pub fn height(&self) -> u32 {
        return self.base.height();
    }
    ///Returns a BlurArea structure, used to dynamically show which part of the image is going to be cropped
    ///The BlurArea structure is manipulated directly to modify the crop area
    ///Takes as parameters the position of the left-upper angle of the crop area and its size
    pub fn blur_area(&self, x: u32, y: u32, width: u32, height: u32) -> BlurArea {
        let mut image = self.base.clone();
        for layer in &self.layers {
            image::imageops::overlay(&mut image, &layer.layer, 0, 0);
        }
        let mut blur = image.clone();
        blur = blur.brighten(100);
        BlurArea::new(image, blur, (x,y), (width,height))
    }
    ///Crops the image given a BlurArea previously obtained via the blur_area method
    ///Once the crop is done it's not possible to go back and the layers get merged 
    ///together and so also those modifications cannot be undone
    pub fn crop(&mut self, crop_area: BlurArea) {
        let ((x,y), (width, height)) = crop_area.get_crop_data();
        let cropped = crop_area.show().crop(x, y, width, height);
        self.base = cropped;
        self.layers.clear();
        
    }
    ///Flips the image orizontally
    pub fn flip_horizontally(&mut self) {
        let flipped = self.base.fliph();
        self.base = flipped;
        for i in 0..self.layers.len() {
            let flipped = self.layers[i].layer.fliph();
            self.layers[i].layer = flipped;
        }
    }
    ///Flips the image vertically
    pub fn flip_vertically(&mut self) {
        let flipped = self.base.flipv();
        self.base = flipped;
        for i in 0..self.layers.len() {
            let flipped = self.layers[i].layer.flipv();
            self.layers[i].layer = flipped;
        }
    }
    ///Rotates the image 90 degree clockwise
    pub fn rotate90cv(&mut self) {
        let rotated = self.base.rotate90();
        self.base = rotated;
        for i in 0..self.layers.len() {
            let rotated = self.layers[i].layer.rotate90();
            self.layers[i].layer = rotated;
        }
        let tmp = self.size.0;
        self.size.0 = self.size.1;
        self.size.1 = tmp;
    }
    ///Rotates the image 180 degree clockwise
    pub fn rotate180cv(&mut self) {
        let rotated = self.base.rotate180();
        self.base = rotated;
        for i in 0..self.layers.len() {
            let rotated = self.layers[i].layer.rotate180();
            self.layers[i].layer = rotated;
        }
    }
    ///Rotates the image 270 degree clockwise
    pub fn rotate270cv(&mut self) {
        let rotated = self.base.rotate270();
        self.base = rotated;
        for i in 0..self.layers.len() {
            let rotated = self.layers[i].layer.rotate270();
            self.layers[i].layer = rotated;
        }
        let tmp = self.size.0;
        self.size.0 = self.size.1;
        self.size.1 = tmp;
    }
    ///Creates an additional layer containing a filled ellipse with given center, major semiaxis, minor semiaxis and color
    pub fn draw_filled_ellipse(&mut self, center: (i32, i32), width_radius: i32, height_radius: i32, color: &Color) {
        let mut layer = RgbaImage::new(self.size.0, self.size.1);
        drawing::draw_filled_ellipse_mut(&mut layer, center, width_radius, height_radius, color.color);
        let layer = Layer::new(image::DynamicImage::ImageRgba8(layer),LayerType::Shape);
        self.layers.push_front(layer);
    }
    ///Creates an additional layer containing an empty ellipse with given center, major semiaxis, minor semiaxis and color
    pub fn draw_empty_ellipse(&mut self, center: (i32, i32), width_radius: i32, height_radius: i32, color: &Color) {
        let mut layer = RgbaImage::new(self.size.0, self.size.1);
        drawing::draw_hollow_ellipse_mut(&mut layer, center, width_radius, height_radius, color.color);
        let layer = Layer::new(image::DynamicImage::ImageRgba8(layer),LayerType::Shape);
        self.layers.push_front(layer);
    }
    ///Creates an additional layer containing a filled rectangle given the upper-left corner, its dimensions and
    ///its color
    pub fn draw_filled_rectangle(&mut self, corner: (i32, i32), dimensions: (u32, u32), color: &Color) {
        let mut layer = RgbaImage::new(self.size.0, self.size.1);
        let rect = imageproc::rect::Rect::at(corner.0, corner.1).of_size(dimensions.0, dimensions.1);
        drawing::draw_filled_rect_mut(&mut layer, rect, color.color);
        let layer = Layer::new(image::DynamicImage::ImageRgba8(layer),LayerType::Shape);
        self.layers.push_front(layer);
    }
    ///Creates an additional layer containing an empty rectangle given the upper-left corner, its dimensions and
    ///its color
    pub fn draw_empty_rectangle(&mut self, corner: (i32, i32), dimensions: (u32, u32), color: &Color) {
        let mut layer = RgbaImage::new(self.size.0, self.size.1);
        let rect = imageproc::rect::Rect::at(corner.0, corner.1).of_size(dimensions.0, dimensions.1);
        drawing::draw_hollow_rect_mut(&mut layer, rect, color.color);
        let layer = Layer::new(image::DynamicImage::ImageRgba8(layer),LayerType::Shape);
        self.layers.push_front(layer);
    }
    ///Draws a line given the initial and final point and the color of the line
    pub fn draw_line(&mut self, start: (i32, i32), end: (i32, i32), color: &Color) {
        let mut layer = RgbaImage::new(self.size.0, self.size.1);
        let start = (start.0 as f32, start.1 as f32);
        let end = (end.0 as f32, end.1 as f32);
        drawing::draw_line_segment_mut(&mut layer, start, end, color.color);
        let layer = Layer::new(image::DynamicImage::ImageRgba8(layer),LayerType::Shape);
        self.layers.push_front(layer);
    }
    ///Draws a polygon given the color and the structure Polygon describing it
    pub fn draw_polygon(&mut self, polygon: Polygon, color: &Color) {
        let mut layer = RgbaImage::new(self.size.0, self.size.1);
        drawing::draw_polygon_mut(&mut layer, &polygon.vertices, color.color);
        let layer = Layer::new(image::DynamicImage::ImageRgba8(layer),LayerType::Shape);
        self.layers.push_front(layer);
    }
    ///Puts a text in the image given the text to write, its color, the position of the upper-left corner,
    ///the font and the font size 
    pub fn put_text<'a>(&mut self, start: (i32, i32), color: &Color, text: &str, font_size: f32, font: &'a rusttype::Font<'a>) {
        let mut layer = RgbaImage::new(self.size.0, self.size.1);
        drawing::draw_text_mut(&mut layer, color.color, start.0, start.1, rusttype::Scale::uniform(font_size), font, text);
        let layer = Layer::new(image::DynamicImage::ImageRgba8(layer),LayerType::Text);
        self.layers.push_front(layer);
    }
    ///Initializes a Layer for free-hand drawing. Return an empty layer on which
    ///it is possible to draw
    pub fn free_hand_draw_init(&self) -> Layer{
        let layer = RgbaImage::new(self.size.0, self.size.1);
        let layer = Layer::new(image::DynamicImage::ImageRgba8(layer),LayerType::FreeHandDrawing);
        layer
    }
    ///Finalizes the free-hand drawing layer and puts it with the others. Takes as parameter the previously
    ///defines Layer used for drawing
    pub fn free_hand_draw_set(&mut self, layer: Layer) {
        self.layers.push_front(layer);
    }
    ///Draws a point on a previously deifned Layer (returned by free_hand_draw_init) given the point position
    ///and its color. Takes a mutable reference to such Layer
    pub fn draw_point(layer: &mut Layer, x: i32, y: i32, color: &Color, size: i32) {
        drawing::draw_filled_circle_mut(&mut layer.layer, (x,y), size, color.color);
    }
    ///Remove the most recent created layer
    pub fn undo(&mut self) {
        self.layers.pop_front();
    }
    ///Saves the image given the extension. This is used if no name is provided and the given name is
    ///unnamed_N, where N is an incremental counter starting from 0
    pub fn save(&self, extension: Extensions) -> Result<(),ImageManipulationError> {

        let path = match extension {
            Extensions::JPG => {
                format!("unnamed_{}.jpg",UNNAMED_COUNTER.fetch_add(1, Ordering::SeqCst))
            }
            Extensions::PNG => {
                format!("unnamed_{}.png",UNNAMED_COUNTER.fetch_add(1, Ordering::SeqCst))
            }
            Extensions::GIF => {
                format!("unnamed_{}.gif",UNNAMED_COUNTER.fetch_add(1, Ordering::SeqCst))
            }
        };
        let _file = File::create(&path)?;
        let mut image  = self.base.clone();
        for layer in &self.layers {
            image::imageops::overlay(&mut image, &layer.layer, 0, 0);
        }
        image.save(path)?;
        
        Ok(())
    }
    ///Saves the image given the extension and the name one want to give it. The name includes also 
    ///the path.
    pub fn save_as(&self, name: &str, extension: Extensions) -> Result<(),ImageManipulationError> {

        let path = match extension {
            Extensions::JPG => {
                format!("{}.jpg",name)
            }
            Extensions::PNG => {
                format!("{}.png",name)
            }
            Extensions::GIF => {
                format!("{}.gif",name)
            }
        };
        let _file = File::create(&path)?;
        let mut image  = self.base.clone();
        for layer in &self.layers {
            image::imageops::overlay(&mut image, &layer.layer, 0, 0);
        }
        image.save(path)?;
        
        Ok(())
    }
    ///Returns the image with all the layers stacked
    ///The original image is cloned, and all the layers are merged
    pub fn show(&self) -> DynamicImage {
        let mut image  = self.base.clone();
        for layer in &self.layers {
            image::imageops::overlay(&mut image, &layer.layer, 0, 0);
        }
        image
    }
    ///Copies the image to the clipboard
    ///An error is returned if the operation is not successfull
    pub fn copy_to_clipboard(&self, clipboard: &mut arboard::Clipboard) -> Result<(),ImageManipulationError> {
        let image = self.show();
        let image_cb = arboard::ImageData{
            width: image.width() as usize,
            height: image.height() as usize,
            bytes: Cow::from(image.as_bytes())
        };
        clipboard.set_image(image_cb)?;
        Ok(())
    }

}