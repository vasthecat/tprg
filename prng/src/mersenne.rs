use crate::prgenerator::PRGenerator;

const W: u32 = 32;
const N: usize = 624;
const M: usize = 397;
const R: u32 = 31;
const A: u32 = 0x9908B0DF;
const U: u32 = 11;
const D: u32 = 0xFFFFFFFF;
const S: u32 = 7;
const B: u32 = 0x9D2C5680;
const T: u32 = 15;
const C: u32 = 0xEFC60000;
const L: u32 = 18;
const F: u32 = 1812433253;

const LOWER_MASK: u32 = (1 << R) - 1;
const UPPER_MASK: u32 = !LOWER_MASK;

pub struct MersennePRG {
    mt: [u32; N],
    index: usize,
    modulo: u32,
}

impl MersennePRG {
    pub fn new(modulo: u32, x: u32) -> MersennePRG {
        let mut prg = MersennePRG {
            mt: [0; N],
            index: N + 1,
            modulo,
        };
        prg.seed_mt(x);
        return prg;
    }

    fn seed_mt(&mut self, seed: u32) {
        self.index = N;
        self.mt[0] = seed;
        for i in 1..=N - 1 {
            let a = (F % self.modulo) as u64;
            let b = ((self.mt[i - 1] ^ (self.mt[i - 1] >> (W - 2)))
                % self.modulo) as u64;
            let c = (a * b + i as u64) % self.modulo as u64;
            self.mt[i] = c as u32;
        }
    }

    fn twist(&mut self) {
        for i in 0..N {
            let x =
                (self.mt[i] & UPPER_MASK) | (self.mt[(i + 1) % N] & LOWER_MASK);
            let x_a = if x % 2 == 0 { x >> 1 } else { (x >> 1) ^ A };
            self.mt[i] = self.mt[(i + M) % N] ^ x_a;
        }
        self.index = 0;
    }
}

impl PRGenerator for MersennePRG {
    fn next(&mut self) -> u32 {
        if self.index == N {
            self.twist();
        } else if self.index > N {
            panic!("Генератор не был инициализирован");
        }

        let mut y = self.mt[self.index];
        y = y ^ ((y >> U) & D);
        y = y ^ ((y << S) & B);
        y = y ^ ((y << T) & C);
        y = y ^ (y >> L);

        self.index += 1;
        return y % self.modulo;
    }
}
