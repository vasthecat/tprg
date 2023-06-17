use crate::prgenerator::{PRGenerator, MOD};

pub struct Rc4PRG {
    k: Vec<u32>,
    s: Vec<usize>,
    i: usize,
    j: usize,
}

impl Rc4PRG {
    pub fn new(coeff: Vec<u32>) -> Rc4PRG {
        let mut prg = Rc4PRG {
            k: coeff,
            s: vec![0; 256],
            i: 0,
            j: 0,
        };
        for i in 0..=255 {
            prg.s[i] = i;
        }
        let mut j = 0;
        for i in 0..=255 {
            j = (j + prg.s[i] + prg.k[i] as usize) % 256;
            prg.s.swap(i, j);
        }
        prg
    }
}

impl PRGenerator for Rc4PRG {
    fn next(&mut self) -> u32 {
        self.i = (self.i + 1) % 256;
        self.j = (self.j + self.s[self.i]) % 256;
        self.s.swap(self.i, self.j);
        let t = (self.s[self.i] + self.s[self.j]) % 256;
        let k: u32 = self.s[t].try_into().unwrap();
        k % MOD
    }
}
