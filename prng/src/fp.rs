use crate::prgenerator::PRGenerator;
use bitvec::prelude::*;

pub struct FiveParamPRG {
    q1: usize,
    q2: usize,
    q3: usize,
    w: usize,
    vec: BitVec,
}

impl FiveParamPRG {
    pub fn new(
        p: u64,
        q1: u64,
        q2: u64,
        q3: u64,
        w: u64,
        xs: Vec<u64>,
    ) -> FiveParamPRG {
        let mut bv = bitvec![0; p as usize];
        for i in 0..xs.len() {
            *bv.get_mut(i).unwrap() = if xs[i] == 0 { false } else { true };
        }
        FiveParamPRG {
            q1: q1 as usize,
            q2: q2 as usize,
            q3: q3 as usize,
            w: w as usize,
            vec: bv,
        }
    }
}

impl PRGenerator for FiveParamPRG {
    fn next(&mut self) -> u64 {
        let mut word = BitVec::<u64, Msb0>::new();
        for _ in 0..self.w {
            let bit = self.vec[0]
                ^ self.vec[self.q1 - 1]
                ^ self.vec[self.q2 - 1]
                ^ self.vec[self.q3 - 1];
            self.vec.shift_left(1);
            *self.vec.last_mut().unwrap() = bit;
            word.push(bit);
        }
        return word.load_be::<u64>();
    }
}
