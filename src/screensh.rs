pub mod screensh_errors;

use std::fs::File;
use std::io::Write;
use screensh_errors::ScreenshotError;
use eframe::egui::Vec2;

pub struct Screen {
    screen: screenshots::Screen,
}

pub struct Screenshot {
    image: screenshots::Image,
}

impl Screen {
    fn new(screen: screenshots::Screen) -> Self {
        Self {
            screen: screen
        }
    }
    pub fn get_screens() -> Result<Vec<Screen>,ScreenshotError> {
        match screenshots::Screen::all() {
            Ok(screens) => {
                let mut v = vec![];
                for screen in screens {
                    v.push(Screen::new(screen));
                }
                return Ok(v);
            }
            Err(_) => Err(ScreenshotError::ScreenRetvError)
        }
    }
    pub fn capture(&self) -> Result<Screenshot,ScreenshotError> {
        match self.screen.capture() {
            Ok(image) => {
                return Ok(Screenshot::new(image));
            } 
            Err(_) => Err(ScreenshotError::ScreenCaptureError)
        }
    }
    pub fn get_size(&self) -> Vec2 {
        Vec2::new(
            self.screen.display_info.height as f32,
            self.screen.display_info.width as f32
        )
    }
    
}

impl Screenshot {
    fn new(image: screenshots::Image) -> Self {
        Self { image: image }
    }
    fn serialize(&self) -> Result<Vec<u8>, ScreenshotError> {
        match self.image.to_png(None) {
            Ok(image) => {
                return Ok(image);
            }
            Err(_) => Err(ScreenshotError::ImageProcessError)
        }
    }
    pub fn save(&self) -> Result<(),ScreenshotError> {
        let mut file = File::create(".tmp.png")?;
        let buffer = self.serialize()?;
        file.write(&buffer)?;

        Ok(())
    }
    
}