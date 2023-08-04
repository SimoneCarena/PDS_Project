mod screensh;
mod image_proc;
mod load_fonts;
mod cursor_scaling;

use std::fs;
use cursor_scaling::*;
use image_proc::extensions::Extensions;

use crate::image_proc::Image;

fn main() {

    let mut screen = screensh::Screen::get_screens().unwrap();
    let screen = &mut screen[0];
    let image = screen.capture().unwrap();
    image.save().unwrap();

    let mut image = image_proc::Image::open(".tmp.png").unwrap();
    let w = image.width();
    let h = image.height();
    image.save(Extensions::PNG).unwrap();
    
    let mut blur = image.blur_area(0, 0, w, h);
    let ((x,y),(w,h)) = blur.get_crop_data();
    println!("{} {} {} {}",x,y,w,h);

    let ((nx,ny),(nw,nh)) = get_new_area((x,y), (1000,1000), (x,y), (w,h), Corner::UpLeft);
    println!("{} {} {} {}",nx,ny,nw,nh);
    blur.resize((nx,ny), (nw,nh));
    Image::from_image(blur.show()).save(Extensions::PNG).unwrap();

    let ((x,y),(w,h)) = blur.get_crop_data();
    let ((nx,ny),(nw,nh)) = get_new_area((x+w,y+h), (x+w-200,y+h-200), (x,y), (w,h), Corner::DownRight);
    println!("{} {} {} {}",nx,ny,nw,nh);
    blur.resize((nx,ny), (nw,nh));
    Image::from_image(blur.show()).save(Extensions::PNG).unwrap();

    let ((x,y),(w,h)) = blur.get_crop_data();
    let ((nx,ny),(nw,nh)) = get_new_area((x+w,y), (x+w-200,y+200), (x,y), (w,h), Corner::UpRight);
    println!("{} {} {} {}",nx,ny,nw,nh);
    blur.resize((nx,ny), (nw,nh));
    Image::from_image(blur.show()).save(Extensions::PNG).unwrap();

    let ((x,y),(w,h)) = blur.get_crop_data();
    let ((nx,ny),(nw,nh)) = get_new_area((x,y+h), (x+500,y+h-500), (x,y), (w,h), Corner::DownLeft);
    println!("{} {} {} {}",nx,ny,nw,nh);
    blur.resize((nx,ny), (nw,nh));
    Image::from_image(blur.show()).save(Extensions::PNG).unwrap();

    image.crop(blur);
    image.save(Extensions::PNG).unwrap();

    fs::remove_file(".tmp.png").unwrap();

}
