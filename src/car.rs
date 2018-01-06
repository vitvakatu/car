use support::{DIR, Coord};

pub struct Car {
    pub direction : DIR,
    pub position  : Coord,
    pub vec_speed : Coord,
    pub val_speed : u32,
}

impl Car {
    pub fn new(x : i32, y : i32) -> Car {
        Car {
            position      : Coord::new(x, y),
            direction     : DIR::U,
            vec_speed: Coord {
                x: 0,
                y: -1,
            },
            val_speed     :  0,

        }
    }
    fn change_direction(&mut self, direction : DIR) {
        self.direction = direction;
        match direction {
            DIR::U  => {self.vec_speed.x =  0; self.vec_speed.y = -1;}
            DIR::UR => {self.vec_speed.x =  1; self.vec_speed.y = -1;}
            DIR::R  => {self.vec_speed.x =  1; self.vec_speed.y =  0;}
            DIR::RD => {self.vec_speed.x =  1; self.vec_speed.y =  1;}
            DIR::D  => {self.vec_speed.x =  0; self.vec_speed.y =  1;}
            DIR::DL => {self.vec_speed.x = -1; self.vec_speed.y =  1;}
            DIR::L  => {self.vec_speed.x = -1; self.vec_speed.y =  0;}
            DIR::LU => {self.vec_speed.x = -1; self.vec_speed.y = -1;}
        }
    }
}
