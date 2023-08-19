pub mod font_errors;

use std::path::{PathBuf, Path};
use std::collections::{HashMap, BTreeMap};
use std::fs;
use rusttype::Font;
use std::io::Read;
use font_errors::LoadFontError;
use home;

pub fn load_fonts() -> Result<BTreeMap<String, Font<'static>>, LoadFontError>{

    let font_extensions = vec!["otf".to_string(),"ttf".to_string(),"woff".to_string()];
    let mut fonts = BTreeMap::new();

    if cfg!(target_os = "windows") {
        let path = PathBuf::from(r"C:\Windows\Fonts\");
        let dir = fs::read_dir(&path)?;
        for file in dir {
            let file = file?;
            if file.file_type()?.is_dir() {
                continue;
            }
            let name = file.file_name();
            let extension = Path::new(&name).extension();
            let extension = match extension {
                Some(e) => e.to_owned().into_string().map_err(|_| LoadFontError::InvalidFileNameError)?,
                None => continue
            };
            let name = Path::new(&name).file_stem()
                .ok_or(LoadFontError::InvalidFileNameError)?
                .to_owned()
                .into_string()
                .map_err(|_| LoadFontError::InvalidFileNameError)?;
            if !font_extensions.contains(&extension) {
                continue;
            }
            let path = file.path();
            let mut buf: Vec<u8> = Vec::new();
            fs::File::open(path)?.read_to_end(&mut buf)?;
            let font:Font<'static> = match Font::try_from_vec(buf){
                Some(f) => f,
                None => return Err(LoadFontError::FontConversionError)
            };
            fonts.insert(name, font);
        }
        return Ok(fonts);
    } else if cfg!(target_os = "macos"){
        let mut folders = Vec::new();
        let mut home = home::home_dir().ok_or(LoadFontError::OSError)?;
        home.push("Library/Fonts");
        folders.push(home);
        folders.push(PathBuf::from("/Library/Fonts"));
        let mut dirs = Vec::new();
        for path in folders {
            let d = fs::read_dir(path);
            if d.is_err() {
                continue;
            }
            dirs.push(d?);
        }
        if dirs.len() == 0 {
            return Err(LoadFontError::FontSourceError);
        }
        for dir in dirs {
            for file in dir {
                let file = file?;
                if file.file_type()?.is_dir() {
                    continue;
                }
                let name = file.file_name();
                let extension = Path::new(&name).extension();
                let extension = match extension {
                    Some(e) => e.to_owned().into_string().map_err(|_| LoadFontError::InvalidFileNameError)?,
                    None => continue
                };
                let name = Path::new(&name).file_stem()
                    .ok_or(LoadFontError::InvalidFileNameError)?
                    .to_owned()
                    .into_string()
                    .map_err(|_| LoadFontError::InvalidFileNameError)?;
                let path = file.path();
                if !font_extensions.contains(&extension) {
                    continue;
                }
                let mut buf: Vec<u8> = Vec::new();
                fs::File::open(path)?.read_to_end(&mut buf)?;
                let font:Font<'static> = match Font::try_from_vec(buf){
                    Some(f) => f,
                    None => return Err(LoadFontError::FontConversionError)
                };
                fonts.insert(name, font);
            }
        }
    } else if cfg!(target_os = "linux"){
        let mut folders = Vec::new();
        let mut home = home::home_dir().ok_or(LoadFontError::OSError)?;
        home.push(".local/share/fonts");
        folders.push(home);
        folders.push(PathBuf::from("/usr/share/fonts"));
        folders.push(PathBuf::from("/usr/local/share/fonts"));
        //println!("{:?}",folders);
        let mut dirs = Vec::new();
        for path in folders {
            let d = fs::read_dir(path);
            if d.is_err() {
                continue;
            }
            dirs.push(d?);
        }
        if dirs.len() == 0 {
            return Err(LoadFontError::FontSourceError);
        }
        for dir in dirs {
            for file in dir {
                let file = file?;
                if file.file_type()?.is_dir() {
                    continue;
                }
                let name = file.file_name();
                let extension = Path::new(&name).extension();
                let extension = match extension {
                    Some(e) => e.to_owned().into_string().map_err(|_| LoadFontError::InvalidFileNameError)?,
                    None => continue
                };
                let name = Path::new(&name).file_stem()
                    .ok_or(LoadFontError::InvalidFileNameError)?
                    .to_owned()
                    .into_string()
                    .map_err(|_| LoadFontError::InvalidFileNameError)?;
                let path = file.path();
                if !font_extensions.contains(&extension) {
                    continue;
                }
                let mut buf: Vec<u8> = Vec::new();
                fs::File::open(path)?.read_to_end(&mut buf)?;
                let font:Font<'static> = match Font::try_from_vec(buf){
                    Some(f) => f,
                    None => return Err(LoadFontError::FontConversionError)
                };
                fonts.insert(name, font);
            }
        }
    } else {
        return Err(LoadFontError::OSError);
    };

    Ok(fonts)

}

pub fn load_fonts_fallback() -> Result<BTreeMap<String, Font<'static>>, LoadFontError> {
    let mut fonts = BTreeMap::new();
    let font_extensions = vec!["otf".to_string(),"ttf".to_string(),"woff".to_string()];

    let mut path = PathBuf::from(".");
    path.push("fonts");
    let dir = fs::read_dir(&path)?;

    for file in dir {
        let file = file?;
        if file.file_type()?.is_dir(){
            continue;
        }
        let name = file.file_name();
        let extension = Path::new(&name).extension();
        let extension = match extension {
            Some(e) => e.to_owned().into_string().map_err(|_| LoadFontError::InvalidFileNameError)?,
            None => "".to_string()
        };
        let name = Path::new(&name).file_stem()
            .ok_or(LoadFontError::InvalidFileNameError)?
            .to_owned()
            .into_string()
            .map_err(|_| LoadFontError::InvalidFileNameError)?;
        if !font_extensions.contains(&extension) {
            continue;
        }
        let path = file.path();
        let mut buf: Vec<u8> = Vec::new();
        fs::File::open(path)?.read_to_end(&mut buf)?;
        let font:Font<'static> = match Font::try_from_vec(buf){
            Some(f) => f,
            None => return Err(LoadFontError::FontConversionError)
        };
        fonts.insert(name, font);
    }
    

    Ok(fonts)
}