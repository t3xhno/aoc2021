pub struct Submarine {
    hor: i32,
    ver: i32,
    aim: i32,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine { hor: 0, ver: 0, aim: 0 }
    }

    pub fn move_part_1(&mut self, op: u8, i: i32) {
        match op {
            b'f' => self.hor += i,
            b'd' => self.ver += i,
            _ => self.ver -= i,
        }
    }

    pub fn move_part_2(&mut self, op: u8, i: i32) {
        match op {
            b'f' => {
                self.hor += i;
                self.ver += self.aim * i;
            },
            b'd' => self.aim += i,
            _ => self.aim -= i,
        }
    }

    pub fn dist(&self) -> i32 {
        self.hor * self.ver
    }
}
