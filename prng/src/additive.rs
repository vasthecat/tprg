use crate::prgenerator::PRGenerator;
use std::collections::VecDeque;

pub struct AdditivePRG {
    m: u32,
    j: usize,
    k: usize,
    n: usize,
    vec: VecDeque<u32>,
}

impl AdditivePRG {
    pub fn new(m: u32, j: usize, k: usize, init: Vec<u32>) -> Self {
        Self {
            m,
            j,
            k,
            n: init.len(),
            vec: init.into(),
        }
    }
}

impl PRGenerator for AdditivePRG {
    fn next(&mut self) -> u32 {
        let x =
            (self.vec[self.n - self.j] + self.vec[self.n - self.k]) % self.m;
        self.vec.push_back(x);
        self.vec.pop_front();
        return x;
    }
}
