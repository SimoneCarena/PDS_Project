use std::{collections::HashMap, path::{PathBuf, Path}, ffi::OsString};
use image::DynamicImage;

pub fn load_borders() -> Result<HashMap<String,DynamicImage>,std::io::Error>{

    let mut borders:HashMap<String,DynamicImage> = HashMap::new();
    let path = PathBuf::from("assets");
    let files_name = [
        OsString::from("bl_corner.png"),
        OsString::from("br_corner.png"),
        OsString::from("tr_corner.png"),
        OsString::from("tl_corner.png")
    ];
    for file in std::fs::read_dir(&path)? {
        let file = file?;
        if file.file_type()?.is_dir() || !files_name.contains(&file.file_name()) {
            continue;
        }
        let path = file.path();
        let name = Path::file_stem(&path).unwrap();
        let image = image::open(&path).unwrap();
        borders.insert(name.to_owned().into_string().unwrap(), image);
    }

    return Ok(borders);

}