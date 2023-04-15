use crate::prgenerator::PRGenerator;
use std::collections::VecDeque;

pub struct AdditivePRG {
    m: u64,
    j: u64,
    k: u64,
    n: usize,
    vec: VecDeque<u64>,
}

impl AdditivePRG {
    pub fn new(m: u64, j: u64, k: u64) -> Self {
        let mut fib = vec![0, 1];
        for i in 0..(j - 2) as usize {
            fib.push(fib[i] + fib[i + 1]);
        }
        Self {
            m,
            j,
            k,
            n: j as usize,
            vec: fib.into(),
        }
    }
}

impl PRGenerator for AdditivePRG {
    fn next(&mut self) -> u64 {
        let x = (self.vec[self.n - self.j as usize] + self.vec[self.n - self.k as usize]) % self.m;
        self.vec.push_back(x);
        self.vec.pop_front();
        return x;
    }
}
