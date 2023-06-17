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
        todo!()
    }
}
