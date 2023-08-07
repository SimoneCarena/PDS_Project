use imageproc::point::Point;

#[derive(Clone)]
pub struct Arrow {
    pub vertices: Vec<Point<i32>>,
}

impl Arrow {
    pub fn up_from_size(center: (i32, i32), size: (i32, i32)) -> Self {
        let pos = (center.0-size.0/2, center.1-size.1/2);

        let v1 = Point::new(pos.0+size.0/2, pos.1);
        let v2 = Point::new(pos.0+size.0, pos.1+size.1/3);
        let v3 = Point::new(pos.0+(size.0*2)/3, pos.1+size.1/3);
        let v4 = Point::new(pos.0+(size.0*2)/3, pos.1+size.1);
        let v5 = Point::new(pos.0+size.0/3, pos.1+size.1);
        let v6 = Point::new(pos.0+size.0/3, pos.1+size.1/3);
        let v7 = Point::new(pos.0, pos.1+size.1/3);

        let vertices = Vec::from(
            [
                v1,
                v2,
                v3,
                v4,
                v5,
                v6,
                v7
            ]
        );

        Self {
            vertices: vertices,
        }
        
    }
    pub fn right_from_size(center: (i32, i32), size: (i32, i32)) -> Self {
        let pos = (center.0-size.0/2, center.1-size.1/2);

        let v1 = Point::new(pos.0+size.0, pos.1+size.1/2);
        let v2 = Point::new(pos.0+(size.0*2)/3, pos.1+size.1);
        let v3 = Point::new(pos.0+(size.0*2)/3, pos.1+(size.1*2)/3);
        let v4 = Point::new(pos.0, pos.1+(size.1*2)/3);
        let v5 = Point::new(pos.0, pos.1+size.1/3);
        let v6 = Point::new(pos.0+(size.0*2)/3, pos.1+size.1/3);
        let v7 = Point::new(pos.0+(size.0*2)/3, pos.1);

        let vertices = Vec::from(
            [
                v1,
                v2,
                v3,
                v4,
                v5,
                v6,
                v7
            ]
        );

        Self {
            vertices: vertices,
        }
        
    }
    pub fn down_from_size(center: (i32, i32), size: (i32, i32)) -> Self {
        let pos = (center.0-size.0/2, center.1-size.1/2);

        let v1 = Point::new(pos.0+size.0/2, pos.1+size.1);
        let v2 = Point::new(pos.0, pos.1+(size.1*2)/3);
        let v3 = Point::new(pos.0+size.0/3,pos.1+(size.1*2)/3);
        let v4 = Point::new(pos.0+size.0/3,pos.1);
        let v5 = Point::new(pos.0+(size.0*2)/3,pos.1);
        let v6 = Point::new(pos.0+(size.0*2)/3,pos.1+(size.1*2)/3);
        let v7 = Point::new(pos.0+size.0,pos.1+(size.1*2)/3);

        let vertices = Vec::from(
            [
                v1,
                v2,
                v3,
                v4,
                v5,
                v6,
                v7
            ]
        );

        Self {
            vertices: vertices,
        }
        
    }
    pub fn left_from_size(center: (i32, i32), size: (i32, i32)) -> Self {
        let pos = (center.0-size.0/2, center.1-size.1/2);

        let v1 = Point::new(pos.0, pos.1+size.1/2);
        let v2 = Point::new(pos.0+size.0/3, pos.1);
        let v3 = Point::new(pos.0+size.0/3, pos.1+size.1/3);
        let v4 = Point::new(pos.0+size.0, pos.1+size.1/3);
        let v5 = Point::new(pos.0+size.0, pos.1+(size.1*2)/3);
        let v6 = Point::new(pos.0+size.0/3, pos.1+(size.1*2)/3);
        let v7 = Point::new(pos.0+size.0/3,pos.1+size.1);

        let vertices = Vec::from(
            [
                v1,
                v2,
                v3,
                v4,
                v5,
                v6,
                v7
            ]
        );

        Self {
            vertices: vertices,
        }
        
    }

}