pub struct Dial {
    pub current: u16,
    pub solution: u16,
}
impl Dial {
    pub fn new() -> Self {
        Self {
            current: 50u16,
            solution: 0u16,
        }
    }

    pub fn add_part1(&mut self, add: u16) {
        let mut res = self.current + add;
        while res > 99 {
            res -= 100;
        }
        if res == 0 {
            self.solution += 1;
        }
        self.current = res;
    }

    pub fn subtract_part1(&mut self, sub: u16) {
        let mut res = self.current as i32;
        res -= sub as i32;
        while res < 0 {
            res += 100;
        }
        if res == 0 {
            self.solution += 1;
        }
        self.current = res as u16;
    }

    pub fn add_part2(&mut self, add: u16) {
        let turns = add % 100;
        let mut clicks = add / 100;

        let mut res = self.current + turns;
        if res > 99 {
            clicks += 1;
            res -= 100;
        }

        self.current = res;
        self.solution += clicks;
    }

    pub fn subtract_part2(&mut self, sub: u16) {
        let turns = sub % 100;
        let mut clicks = sub / 100;

        let mut res = self.current as i32 - turns as i32;
        if self.current != 0 && res <= 0 {
            clicks += 1;
        }
        if res < 0 {
            res += 100;
        }

        self.current = res as u16;
        self.solution += clicks;
    }
}
