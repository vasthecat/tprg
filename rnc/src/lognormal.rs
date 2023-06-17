use crate::normal::NormalDistribution;
use crate::prdistribution::PRDistribution;

pub struct LognormalDistribution {
    p1: f32,
    p2: f32,
}

impl LognormalDistribution {
    pub fn new(p1: f32, p2: f32) -> Self {
        LognormalDistribution { p1, p2 }
    }
}

impl PRDistribution for LognormalDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        let normal_distr = NormalDistribution::new(0.0, 1.0);
        let mut res = Vec::new();
        for i in 0..xs.len() / 2 {
            let x1 = xs[i * 2];
            let x2 = xs[i * 2 + 1];
            let (z1, z2) = normal_distr.norm(m, x1, x2);

            let y1 = self.p1 + (self.p2 - z1).exp();
            let y2 = self.p1 + (self.p2 - z2).exp();

            res.push(y1);
            res.push(y2);
        }
        res
    }
}
