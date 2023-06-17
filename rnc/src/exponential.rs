use crate::prdistribution::PRDistribution;

pub struct ExponentialDistribution {
    p1: f32,
    p2: f32,
}

impl ExponentialDistribution {
    pub fn new(p1: f32, p2: f32) -> Self {
        ExponentialDistribution { p1, p2 }
    }
}

impl PRDistribution for ExponentialDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        todo!()
    }
}
