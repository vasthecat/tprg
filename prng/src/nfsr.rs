use crate::lfsr::LfsrPRG;
use crate::prgenerator::PRGenerator;
use bitvec::prelude::*;

pub struct NfsrPRG {
    r1: LfsrPRG,
    r2: LfsrPRG,
    r3: LfsrPRG,
    w: u32,
}

impl NfsrPRG {
    pub fn new(
        coeffs1: Vec<u32>,
        init1: Vec<u32>,
        coeffs2: Vec<u32>,
        init2: Vec<u32>,
        coeffs3: Vec<u32>,
        init3: Vec<u32>,
        w: u32,
    ) -> NfsrPRG {
        NfsrPRG {
            r1: LfsrPRG::new(coeffs1, init1),
            r2: LfsrPRG::new(coeffs2, init2),
            r3: LfsrPRG::new(coeffs3, init3),
            w,
        }
    }
}

impl PRGenerator for NfsrPRG {
    fn next(&mut self) -> u32 {
        let mut word = BitVec::<u32, Msb0>::new();
        for _ in 0..self.w {
            let bit1 = self.r1.next_bit() > 0;
            let bit2 = self.r2.next_bit() > 0;
            let bit3 = self.r3.next_bit() > 0;
            let bit = bit1 & bit2 ^ bit2 & bit3 ^ bit3;
            word.push(bit);
        }
        return word.load_be::<u32>();
    }
}
