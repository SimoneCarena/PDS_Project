pub enum Corner {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

pub fn cursor_position(relative_reference: (u32, u32), scale: f32) -> (u32, u32) {

    let x = relative_reference.0 as f32;
    let y = relative_reference.1 as f32;

    ((x*scale) as u32, (y*scale) as u32)

}

pub fn get_new_area(start: (u32, u32), end: (u32, u32), old_pos: (u32, u32), old_size: (u32, u32), corner: Corner) -> ((u32,u32),(u32,u32)){

    let distance = (end.0-start.0, end.1-start.1);
    let new_pos: (u32,u32);
    let new_size: (u32,u32);

    match corner {
        Corner::DownLeft => {
            new_size = (old_size.0-distance.0,old_size.1+distance.1);
            new_pos = (old_pos.0+distance.0,old_pos.1);
        },
        Corner::DownRight => {
            new_size = (old_size.0+distance.0,old_size.1+distance.1);
            new_pos = (old_pos.0,old_pos.1);
        },
        Corner::UpLeft => {
            new_size = (old_size.0-distance.0,old_size.1-distance.1);
            new_pos = (old_pos.0+distance.0,old_pos.1+distance.1);
        },
        Corner::UpRight => {
            new_size = (old_size.0+distance.0,old_size.1-distance.1);
            new_pos = (old_pos.0,old_pos.1+distance.1);
        }
    }

    (new_pos, new_size)

}