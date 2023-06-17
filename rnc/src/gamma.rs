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
        todo!()
    }
}
