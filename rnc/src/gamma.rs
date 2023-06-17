use crate::prdistribution::PRDistribution;

pub struct GammaDistribution {
    p1: f32,
    p2: f32,
    p3: f32,
}

impl GammaDistribution {
    pub fn new(p1: f32, p2: f32, p3: f32) -> Self {
        GammaDistribution { p1, p2, p3 }
    }
}

impl PRDistribution for GammaDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        let mut res = Vec::new();
        // если p3 не целый
        if self.p3 - (self.p3 as u32) as f32 > f32::EPSILON {
            todo!("non-integer p3 parameter");
        } else {
            let c = self.p3 as usize;
            let mut i = 0;
            while i < xs.len() {
                let mut val = 1.0;
                for j in 0..c {
                    val *= 1.0 - (xs[i + j] as f32 / m as f32);
                }
                res.push(self.p1 - self.p2 * val.ln());
                i += c;
            }
        }
        res
    }
}
