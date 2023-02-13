use crate::prgenerator::PRGenerator;

pub struct LinearPRG {
    m: u64,
    a: u64,
    c: u64,
    x: u64,
}

impl LinearPRG {
    pub fn new(m: u64, a: u64, c: u64, x_0: u64) -> Self {
        Self { m, a, c, x: x_0 }
    }
}

impl PRGenerator for LinearPRG {
    fn next(&mut self) -> u64 {
        let x = self.x;
        self.x = (self.a * self.x + self.c) % self.m;
        return x;
    }
}
