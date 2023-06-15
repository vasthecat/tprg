use crate::prgenerator::PRGenerator;
use std::collections::VecDeque;

pub struct LfsrPRG {
    coeff: Vec<u32>,
    vec: VecDeque<u32>,
}

impl LfsrPRG {
    pub fn new(coeff: Vec<u32>, init: Vec<u32>) -> LfsrPRG {
        LfsrPRG {
            coeff,
            vec: init.into(),
        }
    }

    pub fn next_bit(&mut self) -> u32 {
        let mut new_x0 = 0;
        for i in 0..self.coeff.len() {
            new_x0 += self.coeff[i] * self.vec[i];
        }
        new_x0 %= 2;
        let new_bit = *self.vec.front().unwrap();
        self.vec.pop_front();
        self.vec.push_back(new_x0);
        new_bit
    }
}

impl PRGenerator for LfsrPRG {
    fn next(&mut self) -> u32 {
        let mut result = 0;
        for i in 0..std::mem::size_of::<u32>() * 8 {
            result |= self.next_bit() << i;
        }
        result % 1024
    }
}
