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
}

impl PRGenerator for LfsrPRG {
    fn next(&mut self) -> u32 {
        fn next_bit(lfsr: &mut LfsrPRG) -> u32 {
            let mut new_x0 = 0;
            for i in 0..lfsr.coeff.len() {
                new_x0 += lfsr.coeff[i] * lfsr.vec[i];
            }
            new_x0 %= 2;
            let new_bit = *lfsr.vec.front().unwrap();
            lfsr.vec.pop_front();
            lfsr.vec.push_back(new_x0);
            new_bit
        }

        let mut result = 0;
        for i in 0..std::mem::size_of::<u32>() * 8 {
            result |= next_bit(self) << i;
        }
        result % 1024
    }
}
