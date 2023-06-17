use crate::prdistribution::PRDistribution;

pub struct TriangleDistribution {
    p1: f32,
    p2: f32,
}

impl TriangleDistribution {
    pub fn new(p1: f32, p2: f32) -> Self {
        TriangleDistribution { p1, p2 }
    }
}

impl PRDistribution for TriangleDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        let m = m as f32;
        let mut res = Vec::new();
        for i in 0..xs.len() / 2 {
            let x1 = xs[i * 2] as f32;
            let x2 = xs[i * 2 + 1] as f32;
            let val = self.p1 + self.p2 * (x1 / m + x2 / m - 1.0);
            res.push(val);
        }
        res
    }
}
