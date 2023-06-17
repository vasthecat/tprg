use crate::prdistribution::PRDistribution;
use std::f32::consts::PI;

pub struct NormalDistribution {
    p1: f32,
    p2: f32,
}

impl NormalDistribution {
    pub fn new(p1: f32, p2: f32) -> Self {
        NormalDistribution { p1, p2 }
    }

    pub fn norm(&self, m: u32, x1: u32, x2: u32) -> (f32, f32) {
        let t1 = (-2.0 * (1.0 - x1 as f32 / m as f32).ln()).sqrt();
        let t2 = 2.0 * PI * (x2 as f32 / m as f32);

        let y1 = self.p1 + self.p2 * t1 * t2.cos();
        let y2 = self.p1 + self.p2 * t1 * t2.sin();

        (y1, y2)
    }
}

impl PRDistribution for NormalDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        let mut res = Vec::new();
        for i in 0..xs.len() / 2 {
            let (y1, y2) = self.norm(m, xs[i * 2], xs[i * 2 + 1]);
            res.push(y1);
            res.push(y2);
        }
        res
    }
}
