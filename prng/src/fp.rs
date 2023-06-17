use crate::prgenerator::{PRGenerator, MOD};
use bitvec::prelude::*;

pub struct FiveParamPRG {
    q1: usize,
    q2: usize,
    q3: usize,
    w: u32,
    vec: BitVec,
}

impl FiveParamPRG {
    pub fn new(
        p: usize,
        q1: usize,
        q2: usize,
        q3: usize,
        w: u32,
        xs: Vec<u32>,
    ) -> FiveParamPRG {
        let mut bv = bitvec![0; p];
        for (i, x) in xs.iter().enumerate() {
            *bv.get_mut(i).unwrap() = *x != 0;
        }
        FiveParamPRG {
            q1,
            q2,
            q3,
            w,
            vec: bv,
        }
    }
}

impl PRGenerator for FiveParamPRG {
    fn next(&mut self) -> u32 {
        let mut word = BitVec::<u32, Msb0>::new();
        for _ in 0..self.w {
            let bit = self.vec[0]
                ^ self.vec[self.q1 - 1]
                ^ self.vec[self.q2 - 1]
                ^ self.vec[self.q3 - 1];
            self.vec.shift_left(1);
            *self.vec.last_mut().unwrap() = bit;
            word.push(bit);
        }
        word.load_be::<u32>() % MOD
    }
}
