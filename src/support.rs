#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DIR { U, UR, R, RD, D, DL, L, LU }

pub struct Coord {
    pub x : i32,
    pub y : i32,
}

impl Coord {
    pub fn new(x : i32, y: i32) -> Coord {
        Coord { x : x, y : y }
    }
}
