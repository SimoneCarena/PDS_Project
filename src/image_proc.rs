pub mod extensions;
pub mod polygon;
pub mod colors;
pub mod image_errors;
pub mod layer;
pub mod blur_area;

use image::{DynamicImage, RgbaImage};
use imageproc::point::Point;
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
use eframe::egui;

///Incremental counter for files whose name is not specified when saved
static UNNAMED_COUNTER: AtomicUsize = AtomicUsize::new(0);

///Structure containing the base screenshot, its size and the additional layers of editing applied to it
#[derive(Debug, Clone)]
pub struct Image {
    layers: VecDeque<DynamicImage>
}

impl Image {
    ///Returns an Image structure, wrapped in a Result, given the path where to retrieve it from. 
    ///In case of failure an ImageManipulationError is returned, with the IOError variant
    pub fn open(path: &str) -> Result<Self,ImageManipulationError> {
        let image = image::open(path)?;
        let mut layers = VecDeque::new();
        layers.push_front(image);
        Ok(
            Self {
                layers: layers
            }
        )
    }
    ///Return the width of the image
    pub fn width(&self) -> u32 {
        return self.layers[0].width();
    }
    ///Returns the height of the image
    pub fn height(&self) -> u32 {
        return self.layers[0].height();
    }
    ///Returns a BlurArea structure, used to dynamically show which part of the image is going to be cropped
    ///The BlurArea structure is manipulated directly to modify the crop area
    ///Takes as parameters the position of the left-upper angle of the crop area and its size
    pub fn blur_area(&self, x: u32, y: u32, width: u32, height: u32) -> BlurArea {
        let mut image = self.layers[0].clone();
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
        self.layers.push_front(cropped);
        
    }
    ///Flips the image orizontally
    pub fn flip_horizontally(&mut self) {
        let flipped = self.layers[0].fliph();
        self.layers.pop_front();
        self.layers.push_front(flipped);
    }
    ///Flips the image vertically
    pub fn flip_vertically(&mut self) {
        let flipped = self.layers[0].flipv();
        self.layers.pop_front();
        self.layers.push_front(flipped)
    }
    ///Rotates the image 90 degree clockwise
    pub fn rotate90cv(&mut self) {
        let rotated = self.layers[0].rotate90();
        self.layers.pop_front();
        self.layers.push_front(rotated);
        
    }
    ///Rotates the image 180 degree clockwise
    pub fn rotate180cv(&mut self) {
        let rotated = self.layers[0].rotate180();
        self.layers.pop_front();
        self.layers.push_front(rotated);
    }
    ///Rotates the image 270 degree clockwise
    pub fn rotate270cv(&mut self) {
        let rotated = self.layers[0].rotate270();
        self.layers.pop_front();
        self.layers.push_front(rotated);
    }
    ///Creates an additional layer containing a filled ellipse with given center, major semiaxis, minor semiaxis and color
    pub fn draw_filled_ellipse(&mut self, center: (i32, i32), width_radius: i32, height_radius: i32, color: &Color) {
        let mut layer = self.layers[0].clone();
        drawing::draw_filled_ellipse_mut(&mut layer, center, width_radius, height_radius, color.color);
        self.layers.push_front(layer);
    }
    ///Creates an additional layer containing an empty ellipse with given center, major semiaxis, minor semiaxis and color
    pub fn draw_empty_ellipse(&mut self, center: (i32, i32), width_radius: i32, height_radius: i32, color: &Color) {
        let mut layer = self.layers[0].clone();
        drawing::draw_hollow_ellipse_mut(&mut layer, center, width_radius, height_radius, color.color);
        self.layers.push_front(layer);
    }
    ///Creates an additional layer containing a filled rectangle given the upper-left corner, its dimensions and
    ///its color
    pub fn draw_filled_rectangle(&mut self, corner: (i32, i32), dimensions: (u32, u32), color: &Color) {
        let mut layer = self.layers[0].clone();
        let rect = imageproc::rect::Rect::at(corner.0, corner.1).of_size(dimensions.0, dimensions.1);
        drawing::draw_filled_rect_mut(&mut layer, rect, color.color);
        self.layers.push_front(layer);
    }
    ///Creates an additional layer containing an empty rectangle given the upper-left corner, its dimensions and
    ///its color
    pub fn draw_empty_rectangle(&mut self, corner: (i32, i32), dimensions: (u32, u32), color: &Color) {
        let mut layer = self.layers[0].clone();
        let rect = imageproc::rect::Rect::at(corner.0, corner.1).of_size(dimensions.0, dimensions.1);
        drawing::draw_hollow_rect_mut(&mut layer, rect, color.color);
        self.layers.push_front(layer);
    }
    ///Draws a line given the initial and final point and the color of the line
    pub fn draw_line(&mut self, start: (i32, i32), end: (i32, i32), color: &Color) {
        let mut layer = self.layers[0].clone();
        let start = (start.0 as f32, start.1 as f32);
        let end = (end.0 as f32, end.1 as f32);
        drawing::draw_line_segment_mut(&mut layer, start, end, color.color);
        self.layers.push_front(layer);
    }
    ///Draws a polygon given the color and the structure Polygon describing it
    pub fn draw_polygon(&mut self, polygon: Polygon, color: &Color) {
        let mut layer = self.layers[0].clone();
        drawing::draw_polygon_mut(&mut layer, &polygon.vertices, color.color);
        self.layers.push_front(layer);
    }
    ///Puts a text in the image given the text to write, its color, the position of the upper-left corner,
    ///the font and the font size 
    pub fn put_text<'a>(&mut self, start: (i32, i32), color: &Color, text: &str, font_size: f32, font: &'a rusttype::Font<'a>) {
        let mut layer = self.layers[0].clone();
        drawing::draw_text_mut(&mut layer, color.color, start.0, start.1, rusttype::Scale::uniform(font_size), font, text);
        self.layers.push_front(layer);
    }
    ///Initializes a Layer for free-hand drawing. Return an empty layer on which
    ///it is possible to draw
    pub fn free_hand_draw_init(&self) -> Layer{
        let layer = self.layers[0].clone();
        let layer = Layer::new(layer,LayerType::FreeHandDrawing);
        layer
    }
    ///Finalizes the free-hand drawing layer and puts it with the others. Takes as parameter the previously
    ///defines Layer used for drawing
    pub fn free_hand_draw_set(&mut self, mut layer: Layer, last: (i32, i32), size: i32, color: &Color ) {
        imageproc::drawing::draw_filled_circle_mut(&mut layer.layer, last, size/2, color.color);
        self.layers.push_front(layer.layer);
    }
    ///Draws a point on a previously deifned Layer (returned by free_hand_draw_init) given the point position
    ///and its color. Takes a mutable reference to such Layer
    pub fn draw_point(layer: &mut Layer, prev: Option<((i32, i32),(i32, i32),(i32, i32))>, current: (i32, i32), size: i32, color: &Color) -> ((i32, i32), (i32, i32), (i32, i32)) {

        let (x1, y1) = (current.0 as f32, current.1 as f32);

        match prev {
            Some((first, second, (x0, y0))) => {

                let x0 = x0 as f32;
                let y0 = y0 as f32;

                let distance = ((x1-x0) as f32, (y1-y0) as f32);
                let m = (((y1-y0) as f32),((x1-x0) as f32));
                let theta = f32::atan2(m.1,m.0);
                let s = f32::sin(theta);
                let c = f32::cos(theta);

                let mut new_first = (first.0 as f32, first.1 as f32);
                let mut new_second = (second.0 as f32, second.1 as f32);

                new_first.0 -= x0;
                new_first.1 -= y0;
                new_second.0 -= x0;
                new_second.1 -= y0;

                let nx1 = new_first.0*c-new_first.1*s;
                let nx2 = new_second.0*c-new_second.1*s;

                let ny1 = new_first.0*s+new_first.1*c;
                let ny2 = new_first.0*s+new_first.1*c;

                new_first.0 = nx1+x0;
                new_first.1 = ny1+y0;
                new_second.0 = nx2+x0;
                new_second.1 = ny2+y0;

                let third = (new_first.0 + distance.0, new_first.1 + distance.1);
                let fourth = (new_second.0 + distance.0, new_second.1 + distance.1);

                let points = Vec::from(
                    if x1>=x0 && y1>=y0 {
                        [
                            Point::new(new_first.0 as i32, new_first.1 as i32),
                            Point::new(first.0 as i32, first.1 as i32),
                            Point::new(third.0 as i32, third.1 as i32),
                            Point::new(fourth.0 as i32, new_second.1 as i32),
                            Point::new(new_second.0 as i32, new_second.1 as i32),
                            Point::new(second.0, second.1)
                        ]
                    } else if x1<x0 && y1>y0 {
                        [
                            Point::new(first.0 as i32, first.1 as i32),
                            Point::new(new_first.0 as i32, new_first.1 as i32),
                            Point::new(third.0 as i32, third.1 as i32),
                            Point::new(fourth.0 as i32, new_second.1 as i32),
                            Point::new(new_second.0 as i32, new_second.1 as i32),
                            Point::new(second.0, second.1)
                        ]
                    } else if x1<x0 && y1<y0{
                        [
                            Point::new(new_first.0 as i32, new_first.1 as i32),
                            Point::new(first.0 as i32, first.1 as i32),
                            Point::new(third.0 as i32, third.1 as i32),
                            Point::new(fourth.0 as i32, new_second.1 as i32),
                            Point::new(new_second.0 as i32, new_second.1 as i32),
                            Point::new(second.0, second.1)
                        ]
                    } else {
                        [
                            Point::new(first.0 as i32, first.1 as i32),
                            Point::new(new_first.0 as i32, new_first.1 as i32),
                            Point::new(third.0 as i32, third.1 as i32),
                            Point::new(fourth.0 as i32, new_second.1 as i32),
                            Point::new(new_second.0 as i32, new_second.1 as i32),
                            Point::new(second.0, second.1)
                        ]
                    }
                );

                let poly = Polygon::from(points);
                imageproc::drawing::draw_polygon_mut(&mut layer.layer, &poly.vertices, color.color);

                return ((third.0 as i32, third.1 as i32),(fourth.0 as i32, fourth.1 as i32),current)
            },
            None => {
                return ((current.0 - size/2, current.1),(current.1+size/2, current.1),current);
            }
        }
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
        let image  = self.layers[0].clone();
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
        let image  = self.layers[0].clone();
        image.save(path)?;
        
        Ok(())
    }
    ///Returns the image with all the layers stacked
    ///The original image is cloned, and all the layers are merged
    pub fn show(&self) -> DynamicImage {
        self.layers[0].clone()
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

pub fn get_image(filepath: &str, ix: u32, iy: u32, iw: u32, ih: u32) -> egui::ImageData {
    let fp = std::path::Path::new(filepath);
    let color_image = load_image_from_path(&fp).unwrap();
    std::fs::remove_file(".tmp.png").unwrap();
    let img = egui::ImageData::from(color_image);
    img
}

pub fn get_image_from_memory(di: DynamicImage, ix: u32, iy: u32, iw: u32, ih: u32) -> egui::ImageData {
    let color_image = load_image_from_memory(di).unwrap();
    let img = egui::ImageData::from(color_image);
    img
}

pub fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

pub fn load_image_from_memory(image_data: DynamicImage) -> Result<egui::ColorImage, image::ImageError> {
    //let image = image::load_from_memory(&image_data)?;
    let size = [image_data.width() as _, image_data.height() as _];
    let image_buffer = image_data.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}