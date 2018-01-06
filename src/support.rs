pub enum DIR { U, UR, R, RD, D, DL, L, LU };

pub struct Coord {
    x : u32,
    y : u32,
}

impl Coord {
    fn new(x : u32, y: u32) -> Coord {
        Coord { x : x, y : y }
    }
}