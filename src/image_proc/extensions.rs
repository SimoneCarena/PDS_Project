use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extensions{
    JPG,
    PNG,
    GIF    
}

impl Display for Extensions{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Extensions::JPG => {
                write!(f, "JPG")
            }
            Extensions::PNG => {
                write!(f, "PNG")
            }
            Extensions::GIF => {
                write!(f, "GIF")
            }
        }
    }
}