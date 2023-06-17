use crate::prdistribution::PRDistribution;

pub struct BinomialDistribution {
    p1: f32,
    p2: f32,
}

impl BinomialDistribution {
    pub fn new(p1: f32, p2: f32) -> Self {
        BinomialDistribution { p1, p2 }
    }
}

impl PRDistribution for BinomialDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        xs.iter()
            .map(|x| {
                let u = *x as f32 / m as f32;
                let mut s = 0.0;
                let mut k = 0.0;

                loop {
                    s += num_integer::binomial(self.p2 as u64, k as u64) as f32
                        * self.p1.powf(k)
                        * (1.0 - self.p1).powf(self.p2 - k);
                    if s > u {
                        return k;
                    }

                    if k < self.p2 - 1.0 {
                        k += 1.0;
                        continue;
                    }
                }
            })
            .collect()
    }
}
