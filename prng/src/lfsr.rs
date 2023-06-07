use crate::prgenerator::PRGenerator;
use std::collections::VecDeque;

pub struct LfsrPRG {
    coeff: Vec<u64>,
    vec: VecDeque<u64>,
}

impl LfsrPRG {
    pub fn new(coeff: &Vec<u64>, init: &Vec<u64>) -> LfsrPRG {
        LfsrPRG {
            coeff: coeff.clone(),
            vec: init.clone().into(),
        }
    }
}

impl PRGenerator for LfsrPRG {
    fn next(&mut self) -> u64 {
        fn next_bit(lfsr: &mut LfsrPRG) -> u64 {
            let mut new_x0 = 0;
            for i in 0..lfsr.coeff.len() {
                new_x0 += lfsr.coeff[i] * lfsr.vec[i];
            }
            new_x0 %= 2;
            let new_bit = *lfsr.vec.front().unwrap();
            lfsr.vec.pop_front();
            lfsr.vec.push_back(new_x0);
            return new_bit;
        }

        let mut result = 0;
        for i in 0..64 {
            result |= next_bit(self) << i;
        }
        return result % 1024;
    }
}
