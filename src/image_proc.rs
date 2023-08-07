pub mod extensions;
pub mod polygon;
pub mod colors;
pub mod image_errors;
pub mod layer;
pub mod blur_area;
mod shape;

use image::{DynamicImage, RgbaImage};
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

///Structure containing the base screenshot, its size and the additional layers of editing applied to it
#[derive(Debug, Clone)]
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
    ///Rotates the image 180 degree clockwise
    pub fn rotate180cv(&mut self) {
        let rotated = self.layers[0].rotate180();
        self.layers.push_front(rotated);
    }
    ///Rotates the image 270 degree clockwise
    pub fn rotate270cv(&mut self) {
        let rotated = self.layers[0].rotate270();
        self.layers.push_front(rotated);
    }
    ///Creates an additional layer containing a filled ellipse with given center, major semiaxis, minor semiaxis and color
    pub fn draw_filled_circle(canva: &mut Layer, base: &mut Layer, center: (i32, i32), diameter: i32, color: &Color) {
        let pos = (center.0-diameter/2, center.1-diameter/2);
        //println!("{:?}",(base.layer.width() as u32, base.layer.height() as u32));
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        drawing::draw_filled_circle_mut(&mut new_canva, center, diameter/2, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(diameter as u32, diameter as u32)));
    }
    ///Creates an additional layer containing an empty ellipse with given center, major semiaxis, minor semiaxis and color
    pub fn draw_empty_circle(canva: &mut Layer, base: &mut Layer, center: (i32, i32), mut diameter: i32, color: &Color, width: i32) {
        let pos_2 = ((center.0-diameter/2) as u32, (center.1-diameter/2) as u32);
        let size_2 = (diameter as u32, diameter as u32);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let mut pos = (center.0-diameter/2, center.1-diameter/2);
        diameter+=width;
        pos.0 -= width/2;
        pos.1 -= width/2;
        for _ in -width/2..width/2{
            pos.0 += 1;
            pos.1 += 1;
            diameter-=2;
            drawing::draw_hollow_circle_mut(&mut new_canva, center, diameter/2, color.color);
        }
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos_2.0, pos_2.1),(size_2.0, size_2.1)));
    }
    ///Creates an additional layer containing a filled rectangle given the upper-left corner, its dimensions and
    ///its color
    pub fn draw_filled_rectangle(canva: &mut Layer, base: &mut Layer, center: (i32, i32), size: (i32, i32), color: &Color) {
        let pos = (center.0-size.0/2, center.1-size.1/2);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let rect = Rect::at(center.0-size.0/2, center.1-size.1/2).of_size(size.0 as u32, size.1 as u32);
        drawing::draw_filled_rect_mut(&mut new_canva, rect, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(size.0 as u32, size.1 as u32)));
    }
    ///Creates an additional layer containing an empty rectangle given the upper-left corner, its dimensions and
    ///its color
    pub fn draw_empty_rectangle(canva: &mut Layer, base: &mut Layer, center: (i32, i32), mut size: (i32, i32), color: &Color, width: i32) {
        let pos_2 = ((center.0-size.0/2) as u32, (center.1-size.1/2) as u32);
        let size_2 = (size.0 as u32, size.1 as u32);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let mut pos = (center.0-size.0/2, center.1-size.1/2);
        size.0 += width;
        size.1 += width;
        pos.0 -= width/2;
        pos.1 -= width/2;
        for _ in -width/2..width/2{
            pos.0 += 1;
            pos.1 += 1;
            size.0-=2;
            size.1-=2;
            let rect = Rect::at(pos.0, pos.1).of_size(size.0 as u32, size.1 as u32);
            drawing::draw_hollow_rect_mut(&mut new_canva, rect, color.color);
        }
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos_2.0, pos_2.1),(size_2.0, size_2.1)));
    }
    pub fn draw_filled_up_arrow(canva: &mut Layer, base: &mut Layer, center: (i32, i32), size: (i32, i32), color: &Color) {
        let pos = (center.0-size.0/2, center.1-size.1/2);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let arrow = Arrow::up_from_size(center, size);
        let poly = Polygon::from(arrow.vertices);
        drawing::draw_polygon_mut(&mut new_canva, &poly.vertices, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(size.0 as u32, size.1 as u32)));
    }
    pub fn draw_filled_right_arrow(canva: &mut Layer, base: &mut Layer, center: (i32, i32), size: (i32, i32), color: &Color) {
        let pos = (center.0-size.0/2, center.1-size.1/2);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let arrow = Arrow::right_from_size(center, size);
        let poly = Polygon::from(arrow.vertices);
        drawing::draw_polygon_mut(&mut new_canva, &poly.vertices, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(size.0 as u32, size.1 as u32)));
    }
    pub fn draw_filled_left_arrow(canva: &mut Layer, base: &mut Layer, center: (i32, i32), mut size: (i32, i32), color: &Color) {
        let pos = (center.0-size.0/2, center.1-size.1/2);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let arrow = Arrow::left_from_size(center, size);
        let poly = Polygon::from(arrow.vertices);
        drawing::draw_polygon_mut(&mut new_canva, &poly.vertices, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(size.0 as u32, size.1 as u32)));
    }
    pub fn draw_filled_down_arrow(canva: &mut Layer, base: &mut Layer, center: (i32, i32), mut size: (i32, i32), color: &Color) {
        let pos = (center.0-size.0/2, center.1-size.1/2);
        let mut new_canva = RgbaImage::new(base.layer.width() as u32, base.layer.height() as u32);
        let arrow = Arrow::down_from_size(center, size);
        let poly = Polygon::from(arrow.vertices);
        drawing::draw_polygon_mut(&mut new_canva, &poly.vertices, color.color);
        canva.layer = DynamicImage::ImageRgba8(new_canva);
        canva.layer_type = LayerType::Shape(((pos.0 as u32, pos.1 as u32),(size.0 as u32, size.1 as u32)));
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
    pub fn draw_point(layer: &mut Layer, prev: Option<((i32, i32), (i32, i32), (i32, i32))>, current: (i32, i32), size: i32, color: &Color) -> ((i32, i32), (i32, i32), (i32, i32)) {
        match prev {
            Some((border_l, border_r, center)) => {

                let x0 = center.0 as f32;
                let y0 = center.1 as f32;
                let x1 = current.0 as f32;
                let y1 = current.1 as f32;

                let m = (y1-y0)/(x1-x0);

                if  m.is_nan() || m.is_infinite() || f32::abs(m)>= 5.0 {
                    let p1 = Point::new(border_l.0, border_l.1);
                    let p2 = Point::new(current.0-size/2,current.1);
                    let p3 = Point::new(current.0+size/2,current.1);
                    let p4 = Point::new(border_r.0,border_r.1);

                    let points = Vec::from(
                        [
                            p1,
                            p2,
                            p3,
                            p4
                        ]
                    );
    
                    let poly = Polygon::from(points);
                    imageproc::drawing::draw_polygon_mut(&mut layer.layer, &poly.vertices, color.color);

                    return ((current.0-size/2,current.1),(current.0+size/2,current.1),current);
                    
                } else if f32::abs(m)<=0.05{
                    let p1 = Point::new(border_l.0, border_l.1);
                    let p2 = Point::new(current.0,current.1-size/2);
                    let p3 = Point::new(current.0,current.1+size/2);
                    let p4 = Point::new(border_r.0,border_r.1);

                    let points = Vec::from(
                        [
                            p1,
                            p2,
                            p3,
                            p4
                        ]
                    );
    
                    let poly = Polygon::from(points);
                    imageproc::drawing::draw_polygon_mut(&mut layer.layer, &poly.vertices, color.color);

                    return ((current.0,current.1-size/2),(current.0,current.1+size/2),current);
                } else {

                    let xl1 = border_l.0;
                    let xr1 = border_r.0;
                    let yl1 = border_l.1;
                    let yr1 = border_r.1;

                    let distance = (current.0 - center.0, current.1-center.1);

                    let xl2 = xl1 + distance.0;
                    let xr2 = xr1 + distance.0;
                    let yl2 = yl1 + distance.1;
                    let yr2 = yr1 + distance.1;

                    let p1: Point<i32>;
                    let p2: Point<i32>;
                    let p3: Point<i32>;
                    let p4: Point<i32>;

                    p1 = Point::new(xl1 as i32,yl1 as i32);
                    p2 = Point::new(xl2 as i32,yl2 as i32);
                    p3 = Point::new(xr2 as i32,yr2 as i32);
                    p4 = Point::new(xr1 as i32,yr1 as i32);

                    let points = Vec::from(
                        [
                            p1,
                            p2,
                            p3,
                            p4
                        ]
                    );
    
                    let poly = Polygon::from(points);
                    imageproc::drawing::draw_polygon_mut(&mut layer.layer, &poly.vertices, color.color);

                    return ((xl2, yl2),(xr2, yr2),current);
                }
            },
            None => {
                let border_l = (current.0-size/2, current.1);
                let border_r = (current.0+size/2, current.1);

                imageproc::drawing::draw_filled_circle_mut(&mut layer.layer, current, size/2, color.color);

                return (border_l, border_r, current);
            }
        }
    }
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
    pub fn rubber_set(&mut self, mut layer: Layer, base: &Layer, last: (i32, i32), size: i32) {
        let color = Color::new(0, 0, 0, 0.0);
        imageproc::drawing::draw_filled_circle_mut(&mut layer.layer, last, size/2, color.color);
        let layer = layer.show_rubber(base);
        self.layers.push_front(layer);
    }
    pub fn rubber(layer: &mut Layer, prev: Option<((i32, i32),(i32, i32),(i32, i32))>, current: (i32, i32), size: i32) -> ((i32, i32), (i32, i32), (i32, i32)) {
        let color = Color::new(0, 0, 0, 0.0);
        Image::draw_point(layer, prev, current, size, &color)
    }

    pub fn highlight_init(&self) -> (Layer, Layer) {
        let base = self.layers[0].clone();
        let base = Layer::new(base,LayerType::FreeHandDrawing);
        let width = base.layer.width();
        let height = base.layer.height();
        let canva = RgbaImage::new(width, height);
        let canva = Layer::new(DynamicImage::ImageRgba8(canva), LayerType::BaseImage);
        (base, canva)
    }
    pub fn highlight_set(&mut self, mut layer: Layer, base: &Layer, last: (i32, i32), size: i32, color: &Color) {
        imageproc::drawing::draw_filled_circle_mut(&mut layer.layer, last, size/2, color.color);
        let layer = layer.show_higlight(base);
        self.layers.push_front(layer);
    }
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
    pub fn save_as(&self, location: &str, name: &str, extension: Extensions) -> Result<(), ImageManipulationError> {    let mut n;
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

    pub fn shape_init(&self, center: (u32, u32), size: (u32, u32)) -> (Layer, Layer) {
        let base = self.layers[0].clone();
        let width = base.width();
        let height = base.height();
        let canva = RgbaImage::new(width,height);
        let pos = (center.0-size.0/2,center.1-size.1/2);

        (Layer::new(base,LayerType::BaseImage), Layer::new(DynamicImage::ImageRgba8(canva),LayerType::Shape((pos,size))))
    }

    pub fn shape_set(&mut self, base: Layer, shape_layer: Layer) {
        let image = shape_layer.draw_shape(&base);
        self.layers.push_front(image);
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