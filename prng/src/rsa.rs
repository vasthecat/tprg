use crate::prgenerator::{PRGenerator, MOD};
use bitvec::prelude::*;

pub struct RsaPRG {
    n: u32,
    e: u32,
    w: u32,
    x: u32,
}

impl RsaPRG {
    pub fn new(n: u32, e: u32, w: u32, x: u32) -> RsaPRG {
        RsaPRG { n, e, w, x }
    }
}

impl PRGenerator for RsaPRG {
    fn next(&mut self) -> u32 {
        fn pow_mod(x: u32, e: u32, m: u32) -> u32 {
            let mut res = 1;
            for _ in 0..e {
                res = (res * x) % m;
            }
            return res;
        }
        let mut word = BitVec::<u32, Msb0>::new();
        for _ in 0..self.w {
            self.x = pow_mod(self.x, self.e, self.n);
            word.push((self.x & 1) != 0);
        }
        return word.load_be::<u32>() % MOD;
    }
}
