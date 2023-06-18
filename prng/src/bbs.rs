use crate::prgenerator::{PRGenerator, MOD};
use bitvec::prelude::*;

const N: u32 = 16637;

pub struct BbsPRG {
    x: u32,
}

impl BbsPRG {
    pub fn new(x: u32) -> BbsPRG {
        BbsPRG {
            x: ((x % N) * (x % N)) % N,
        }
    }
}

impl PRGenerator for BbsPRG {
    fn next(&mut self) -> u32 {
        let mut word = BitVec::<u32, Msb0>::new();
        for _ in 0..std::mem::size_of::<u32>() {
            self.x = ((self.x % N) * (self.x % N)) % N;
            word.push((self.x & 1) != 0);
        }
        word.load_be::<u32>() % MOD
    }
}
