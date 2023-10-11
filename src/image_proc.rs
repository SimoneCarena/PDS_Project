pub mod extensions;
pub mod polygon;
pub mod colors;
pub mod image_errors;
pub mod layer;
pub mod blur_area;
mod shape;

use image::{DynamicImage, RgbaImage};
use imageproc::pixelops::interpolate;
use imageproc::point::Point;
use imageproc::rect::Rect;
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
use shape::Arrow;

///Incremental counter for files whose name is not specified when saved
static UNNAMED_COUNTER: AtomicUsize = AtomicUsize::new(0);

///Structure containing the base screenshot and the additional layers of editing applied to it
#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    base: DynamicImage,
    layers: VecDeque<DynamicImage>
}

impl Image {
    ///Returns an Image structure, wrapped in a Result, given the path where to retrieve it from. 
    ///In case of failure an ImageManipulationError is returned, with the IOError variant
    pub fn open(path: &str) -> Result<Self,ImageManipulationError> {
        let image = image::open(path)?;
        let mut layers = VecDeque::new();
        layers.push_front(image.clone());
        Ok(
            Self {
                base: image,
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
        let image = self.layers[0].clone();
        let mut blur = image.clone();
        blur = blur.brighten(100);
        BlurArea::new(image, blur, (x,y), (width,height))
    }
    ///Crops the image given a BlurArea previously obtained via the blur_area method
    pub fn crop(&mut self, crop_area: BlurArea) {
        let ((x,y), (width, height)) = crop_area.get_crop_data();
        let cropped = crop_area.save().crop(x, y, width, height);
        self.layers.push_front(cropped);
        
    }
    ///Flips the image orizontally
    pub fn flip_horizontally(&mut self) {
        let flipped = self.layers[0].fliph();
        self.layers.push_front(flipped);
    }
    ///Flips the image vertically
    pub fn flip_vertically(&mut self) {
        let flipped = self.layers[0].flipv();
        self.layers.push_front(flipped)
    }
    ///Rotates the image 90 degree clockwise
    pub fn rotate90cv(&mut self) {
        let rotated = self.layers[0].rotate90();
        self.layers.push_front(rotated);
        
    }
    ///Rotates the image 270 degree clockwise
    pub fn rotate270cv(&mut self) {
        let rotated = self.layers[0].rotate270();
        self.layers.push_front(rotated);
    }
    ///Draws a filled circle with given center, diameter and color
    pub fn draw_filled_circle(canva: &mut Layer, base: &mut Layer, center: (i32, i32), diameter: i32, color: &Color) {
        let pos = (center.0-diameter/2, center.1-diameter/2);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        drawing::draw_filled_circle_mut(&mut new_canva, center, diameter/2, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(diameter as u32, diameter as u32)));
    }
    ///Draws an empty circle with given center, diameter, color and contour width
    pub fn draw_empty_circle(canva: &mut Layer, base: &mut Layer, center: (i32, i32), mut diameter: i32, color: &Color, width: i32) {
        let pos_2 = ((center.0-diameter/2) as u32, (center.1-diameter/2) as u32);
        let size_2 = (diameter as u32, diameter as u32);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let mut pos = (center.0-diameter/2, center.1-diameter/2);
        for _ in 0..width{
            drawing::draw_hollow_circle_mut(&mut new_canva, center, diameter/2, color.color);
            pos.0 += 1;
            pos.1 += 1;
            diameter-=2;
        }
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos_2.0, pos_2.1),(size_2.0, size_2.1)));
    }
    ///Draws a filled rectangle given the upper-left corner, its dimensions and
    ///its color
    pub fn draw_filled_rectangle(canva: &mut Layer, base: &mut Layer, center: (i32, i32), size: (i32, i32), color: &Color) {
        let pos = (center.0-size.0/2, center.1-size.1/2);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let rect = Rect::at(center.0-size.0/2, center.1-size.1/2).of_size(size.0 as u32, size.1 as u32);
        drawing::draw_filled_rect_mut(&mut new_canva, rect, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(size.0 as u32, size.1 as u32)));
    }
    ///Draws an empty rectangle given the upper-left corner, its dimensions and
    ///its color
    pub fn draw_empty_rectangle(canva: &mut Layer, base: &mut Layer, center: (i32, i32), mut size: (i32, i32), color: &Color, width: i32) {
        let pos_2 = ((center.0-size.0/2) as u32, (center.1-size.1/2) as u32);
        let size_2 = (size.0 as u32, size.1 as u32);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let mut pos = (center.0-size.0/2, center.1-size.1/2);
        for _ in 0..width{
            let rect = Rect::at(pos.0, pos.1).of_size(size.0 as u32, size.1 as u32);
            drawing::draw_hollow_rect_mut(&mut new_canva, rect, color.color);
            pos.0 += 1;
            pos.1 += 1;
            size.0-=2;
            size.1-=2;
            if size.0 <=0 || size.1 <=0 {
                break;
            }
        }
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos_2.0, pos_2.1),(size_2.0, size_2.1)));
    }
    ///Creates an arrow pointing upward, given its center, size and color
    pub fn draw_filled_up_arrow(canva: &mut Layer, base: &mut Layer, center: (i32, i32), size: (i32, i32), color: &Color) {
        let pos = (center.0-size.0/2, center.1-size.1/2);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let arrow = Arrow::up_from_size(center, size);
        let poly = Polygon::from(arrow.vertices);
        drawing::draw_polygon_mut(&mut new_canva, &poly.vertices, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(size.0 as u32, size.1 as u32)));
    }
    ///Creates an arrow pointing right, given its center, size and color
    pub fn draw_filled_right_arrow(canva: &mut Layer, base: &mut Layer, center: (i32, i32), size: (i32, i32), color: &Color) {
        let pos = (center.0-size.0/2, center.1-size.1/2);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let arrow = Arrow::right_from_size(center, size);
        let poly = Polygon::from(arrow.vertices);
        drawing::draw_polygon_mut(&mut new_canva, &poly.vertices, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(size.0 as u32, size.1 as u32)));
    }
    ///Creates an arrow pointing left, given its center, size and color
    pub fn draw_filled_left_arrow(canva: &mut Layer, base: &mut Layer, center: (i32, i32), size: (i32, i32), color: &Color) {
        let pos = (center.0-size.0/2, center.1-size.1/2);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let arrow = Arrow::left_from_size(center, size);
        let poly = Polygon::from(arrow.vertices);
        drawing::draw_polygon_mut(&mut new_canva, &poly.vertices, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(size.0 as u32, size.1 as u32)));
    }
    ///Creates an arrow pointing downward, given its center, size and color
    pub fn draw_filled_down_arrow(canva: &mut Layer, base: &mut Layer, center: (i32, i32), size: (i32, i32), color: &Color) {
        let pos = (center.0-size.0/2, center.1-size.1/2);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let arrow = Arrow::down_from_size(center, size);
        let poly = Polygon::from(arrow.vertices);
        drawing::draw_polygon_mut(&mut new_canva, &poly.vertices, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(size.0 as u32, size.1 as u32)));
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
    pub fn free_hand_draw_init(&self) -> Layer {
        let layer = self.layers[0].clone();
        let layer = Layer::new(layer,LayerType::FreeHandDrawing);
        layer
    }
    ///Finalizes the free-hand drawing layer and puts it with the others. Takes as parameter the previously
    ///defines Layer used for drawing
    pub fn free_hand_draw_set(&mut self, layer: Layer, _last: (i32, i32), _size: i32, _color: &Color ) {
        self.layers.push_front(layer.layer);
    }
    ///Draws a point on a previously deifned Layer (returned by free_hand_draw_init) given the point position
    ///and its color. Takes a mutable reference to such Layer
    pub fn draw_point(layer: &mut Layer, prev: Option<((i32, i32), (i32, i32), (i32, i32))>, current: (i32, i32), mut size: i32, color: &Color) -> ((i32, i32), (i32, i32), (i32, i32)) {
        
        let c1 = current;
        if size%2==0{
            size = size + 1;
        }

        match prev {
            Some((_, _, c0)) => {

                let d = (c1.0-c0.0, c1.1-c0.1);

                let xc0 = c0.0 as f32;
                let yc0 = c0.1 as f32;
                let xc1 = c1.0 as f32;
                let yc1 = c1.1 as f32;

                if size == 1 {
                    drawing::draw_antialiased_line_segment_mut(
                        &mut layer.layer, 
                        c0, 
                        c1, 
                        color.color, 
                        |a,b,c| {
                            interpolate(a, b, c)
                        }
                    );
                    return (c1,c1,c1);
                }

                let m1 = (yc1-yc0)/(xc1-xc0);

                if m1.is_nan() || m1.is_infinite() {
                    
                    let l1 = (c1.0 - size/2, c1.1);
                    let r1 = (c1.0 + size/2, c1.1);
                    let l0 = (l1.0-d.0, l1.1-d.1);
                    let r0 = (r1.0-d.0, r1.1-d.1);

                    let points = Vec::from(
                        [
                            Point::new(l0.0, l0.1),
                            Point::new(l1.0, l1.1),
                            Point::new(r1.0, r1.1),
                            Point::new(r0.0, r0.1)
                        ]
                    );

                    drawing::draw_polygon_mut(&mut layer.layer, &points, color.color);
                    drawing::draw_filled_circle_mut(&mut layer.layer, c1, size/2, color.color);

                    return (l1,r1,c1);

                } else if m1==0.0 {
                    
                    let l1 = (c1.0, c1.1 - size/2);
                    let r1 = (c1.0, c1.1 + size/2);
                    let l0 = (l1.0-d.0, l1.1-d.1);
                    let r0 = (r1.0-d.0, r1.1-d.1);

                    let points = Vec::from(
                        [
                            Point::new(l0.0, l0.1),
                            Point::new(l1.0, l1.1),
                            Point::new(r1.0, r1.1),
                            Point::new(r0.0, r0.1)
                        ]
                    );

                    drawing::draw_polygon_mut(&mut layer.layer, &points, color.color);
                    drawing::draw_filled_circle_mut(&mut layer.layer, c1, size/2, color.color);

                    return (l1,r1,c1);

                } else {

                    let size = size as f32;

                    let m2 = -1.0/m1;
                    let q2 = yc1-m2*xc1;
                    let r = size/2.0;

                    let m22 = f32::powi(m2,2);
                    let r2 = f32::powi(r as f32,2);
                    let xc12 = f32::powi(xc1,2);
                    let yc12 = f32::powi(yc1,2);
                    let q22 = f32::powi(q2, 2);

                    let xl1 = -(q2 - (q2 + m2*xc1 + m2*f32::sqrt(m22*r2 - m22*xc12 - 2.0*m2*q2*xc1 + 2.0*m2*xc1*yc1 - q22 + 2.0*q2*yc1 + r2 - yc12) + m22*yc1)/(m22 + 1.0))/m2;
                    let yl1 = (q2 + m2*xc1 + m2*f32::sqrt(m22*r2 - m22*xc12 - 2.0*m2*q2*xc1 + 2.0*m2*xc1*yc1 - q22 + 2.0*q2*yc1 + r2 - yc12) + m22*yc1)/(m22 + 1.0);
                    let xr1 = -(q2 - (q2 + m2*xc1 - m2*f32::sqrt(m22*r2 - m22*xc12 - 2.0*m2*q2*xc1 + 2.0*m2*xc1*yc1 - q22 + 2.0*q2*yc1 + r2 - yc12) + m22*yc1)/(m22 + 1.0))/m2;
                    let yr1 = (q2 + m2*xc1 - m2*f32::sqrt(m22*r2 - m22*xc12 - 2.0*m2*q2*xc1 + 2.0*m2*xc1*yc1 - q22 + 2.0*q2*yc1 + r2 - yc12) + m22*yc1)/(m22 + 1.0);

                    let l1 = (xl1 as i32, yl1 as i32);
                    let r1 = (xr1 as i32, yr1 as i32);
                    let l0 = (l1.0-d.0, l1.1-d.1);
                    let r0 = (r1.0-d.0, r1.1-d.1);

                    let points = Vec::from(
                        [
                            Point::new(l0.0, l0.1),
                            Point::new(l1.0, l1.1),
                            Point::new(r1.0, r1.1),
                            Point::new(r0.0, r0.1)
                        ]
                    );

                    drawing::draw_polygon_mut(&mut layer.layer, &points, color.color);
                    drawing::draw_antialiased_line_segment_mut(
                        &mut layer.layer, 
                        l0, 
                        l1, 
                        color.color, 
                        |a,b,c| {
                            interpolate(a, b, c)
                        }
                    );
                    drawing::draw_antialiased_line_segment_mut(
                        &mut layer.layer, 
                        r0, 
                        r1, 
                        color.color, 
                        |a,b,c| {
                            interpolate(a, b, c)
                        }
                    );
                    drawing::draw_filled_circle_mut(&mut layer.layer, c1, (size as i32)/2, color.color);

                    return (l1,r1,c1);

                }
            },
            None => {
                let l0 = (current.0-size/2, current.1);
                let r0 = (current.0+size/2, current.1);

                imageproc::drawing::draw_filled_circle_mut(&mut layer.layer, current, size/2, color.color);

                return (l0, r0, current);
            }
        }
    }
    ///Initializes a Layer for erasing. Return an empty layer on which
    ///it is possible to use the rubber
    pub fn rubber_init(&self, last_crop_data: Option<((u32, u32),(u32, u32))>) -> (Layer, Layer) {
        let layer = self.layers[0].clone();
        let layer = Layer::new(layer,LayerType::BaseImage);
        let mut base = self.base.clone();
        let base = match last_crop_data {
            Some(last_crop_data) => {
                base.crop(last_crop_data.0.0, last_crop_data.0.1, last_crop_data.1.0, last_crop_data.1.1)
            },
            None => {
                base
            }
        };
        let base = Layer::new(base,LayerType::FreeHandDrawing);
        (base, layer)
    }
    ///Sets the rubber modification, finalizing them
    pub fn rubber_set(&mut self, layer: Layer, base: &Layer, _last: (i32, i32), _size: i32) {
        let layer = layer.show_rubber(base);
        self.layers.push_front(layer);
    }
    ///Erases part of the drawings
    pub fn rubber(layer: &mut Layer, prev: Option<((i32, i32),(i32, i32),(i32, i32))>, current: (i32, i32), size: i32) -> ((i32, i32), (i32, i32), (i32, i32)) {
        let color = Color::new(0, 0, 0, 0.0);
        Image::draw_point(layer, prev, current, size, &color)
    }
    ///Initilizes a layer for higliting
    pub fn highlight_init(&self) -> (Layer, Layer) {
        let base = self.layers[0].clone();
        let base = Layer::new(base,LayerType::FreeHandDrawing);
        let width = base.layer.width();
        let height = base.layer.height();
        let canva = RgbaImage::new(width, height);
        let canva = Layer::new(DynamicImage::ImageRgba8(canva), LayerType::BaseImage);
        (base, canva)
    }
    ///Sets the higlight layer, finalizing the modifications
    pub fn highlight_set(&mut self, layer: Layer, base: &Layer, _last: (i32, i32), _size: i32, _color: &Color) {
        //imageproc::drawing::draw_filled_circle_mut(&mut layer.layer, last, size/2, color.color);
        let layer = layer.show_higlight(base);
        self.layers.push_front(layer);
    }
    ///Higlights the layer
    pub fn highlight(layer: &mut Layer, prev: Option<((i32, i32),(i32, i32),(i32, i32))>, current: (i32, i32), size: i32, color: &Color) -> ((i32, i32), (i32, i32), (i32, i32)) {
        Image::draw_point(layer, prev, current, size, color)
    }

    ///Remove the most recent created layer
    pub fn undo(&mut self) -> DynamicImage {
        if self.layers.len() > 1 {
            self.layers.pop_front();
        }
        self.layers[0].clone()
    }
    ///Saves the image given the extension and the name one want to give it. The name includes also 
    ///the path.
    ///If no name is given, the default one is used
    pub fn save_as(&self, location: &str, name: &str, extension: Extensions) -> Result<(), ImageManipulationError> {    
        let n;
        if name.len() == 0{
            n = match extension {
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
        }else{
            n = match extension {
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
        }
        let mut path = String::from(location);
        path.push_str(n.as_str());    let _file = File::create(&path)?;
        let image  = self.layers[0].clone();    image.save(path)?;
        Ok(())
    }
    ///Returns the current image
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
    ///Initilizes two layers for drawing shapes
    pub fn shape_init(&self, center: (u32, u32), size: (u32, u32)) -> (Layer, Layer) {
        let base = self.layers[0].clone();
        let width = base.width();
        let height = base.height();
        let canva = RgbaImage::new(width,height);
        let pos = (center.0-size.0/2,center.1-size.1/2);

        (Layer::new(base,LayerType::BaseImage), Layer::new(DynamicImage::ImageRgba8(canva),LayerType::Shape((pos,size))))
    }
    ///Finalizes the shape drawings
    pub fn shape_set(&mut self, base: Layer, shape_layer: Layer) {
        let image = shape_layer.draw_shape(&base);
        self.layers.push_front(image);
    }

}

pub fn get_image(filepath: &str, _ix: u32, _iy: u32, _iw: u32, _ih: u32) -> egui::ImageData {
    let fp = std::path::Path::new(filepath);
    let color_image = load_image_from_path(&fp).unwrap();
    std::fs::remove_file(".tmp.png").unwrap();
    let img = egui::ImageData::from(color_image);
    img
}

pub fn get_image_from_memory(di: DynamicImage, _ix: u32, _iy: u32, _iw: u32, _ih: u32) -> egui::ImageData {
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