#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Corner {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

pub fn cursor_position(relative_reference: (u32, u32), scale: f32) -> (u32, u32) {

    let x = relative_reference.0 as f32;
    let y = relative_reference.1 as f32;

    ((x/scale) as u32, (y/scale) as u32)

}

pub fn get_new_area(start: (u32, u32), end: (u32, u32), old_pos: (u32, u32), old_size: (u32, u32), image_size: (u32, u32), corner: Corner) -> ((u32,u32),(u32,u32)){

    let start = (start.0 as i32, start.1 as i32);
    let end = (end.0 as i32, end.1 as i32);
    let old_pos = (old_pos.0 as i32, old_pos.1 as i32);
    let old_size = (old_size.0 as i32, old_size.1 as i32);
    let distance = (end.0-start.0, end.1-start.1);
    let mut new_pos: (i32,i32);
    let mut new_size: (i32,i32);
    let limits = (image_size.0 as i32, image_size.1 as i32);

    match corner {
        Corner::DownLeft => {
            if old_size.0-distance.0<=0 || old_size.1+distance.1<=0{
                return ((old_pos.0 as u32, old_pos.1 as u32),(old_size.0 as u32, old_size.1 as u32));
            }
            new_size = (old_size.0-distance.0,old_size.1+distance.1);
            new_pos = (old_pos.0+distance.0,old_pos.1);
        },
        Corner::DownRight => {
            if old_size.0+distance.0<=0 || old_size.1+distance.1<=0{
                return ((old_pos.0 as u32, old_pos.1 as u32),(old_size.0 as u32, old_size.1 as u32));
            }
            new_size = (old_size.0+distance.0,old_size.1+distance.1);
            new_pos = (old_pos.0,old_pos.1);
        },
        Corner::UpLeft => {
            if old_size.0-distance.0<=0 || old_size.1-distance.1<=0 {
                return ((old_pos.0 as u32, old_pos.1 as u32),(old_size.0 as u32, old_size.1 as u32));
            }
            new_size = (old_size.0-distance.0,old_size.1-distance.1);
            new_pos = (old_pos.0+distance.0,old_pos.1+distance.1);
        },
        Corner::UpRight => {
            if old_size.0+distance.0<=0 || old_size.1-distance.1<=0 {
                return ((old_pos.0 as u32, old_pos.1 as u32),(old_size.0 as u32, old_size.1 as u32));
            }
            new_size = (old_size.0+distance.0,old_size.1-distance.1);
            new_pos = (old_pos.0,old_pos.1+distance.1);
        }
    }

    if new_pos.0<0 {
        new_size.0+=new_pos.0;
        new_pos.0 = 0;
    }
    if new_pos.1<0 {
        new_size.1+=new_pos.1;
        new_pos.1 = 0;
    }
    if (new_pos.0+new_size.0)>limits.0 {
        let distance = (new_pos.0+new_size.0)-limits.0;
        new_size.0-=distance;
    }
    if (new_pos.1+new_size.1)>limits.1 {
        let distance = (new_pos.1+new_size.1)-limits.1;
        new_size.1-=distance;
    }

    ((new_pos.0 as u32, new_pos.1 as u32), (new_size.0 as u32, new_size.1 as u32))

}