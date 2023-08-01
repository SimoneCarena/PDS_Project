use imageproc::point::Point;

pub struct Polygon {
    pub vertices: Vec<Point<i32>>
}

impl Polygon {
    pub fn from(vertices: Vec<Point<i32>>) -> Self {
        Self {
            vertices: vertices
        }
    }
}