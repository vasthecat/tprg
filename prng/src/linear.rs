use crate::prgenerator::PRGenerator;

pub struct LinearPRG {
    m: u32,
    a: u32,
    c: u32,
    x: u32,
}

impl LinearPRG {
    pub fn new(m: u32, a: u32, c: u32, x_0: u32) -> Self {
        Self { m, a, c, x: x_0 }
    }
}

impl PRGenerator for LinearPRG {
    fn next(&mut self) -> u32 {
        let x = self.x;
        self.x = (self.a * self.x + self.c) % self.m;
        x
    }
}
