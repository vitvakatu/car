use support::{DIR, Coord};

struct Car {
    direction : DIR,
    position  : Coord,
    vec_speed : Coord,
    val_speed : u32,
}

impl Car {
    fn new(x : u32, y : u32) -> Car {
        Car {
            position      : Coord::new(x, y),
            direction     : DIR::U,
            vec_speed.x   :  0,
            vec_speed.y   : -1,
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
