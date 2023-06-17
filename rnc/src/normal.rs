use crate::prdistribution::PRDistribution;

pub struct NormalDistribution {
    p1: f32,
    p2: f32,
}

impl NormalDistribution {
    pub fn new(p1: f32, p2: f32) -> Self {
        NormalDistribution { p1, p2 }
    }
}

impl PRDistribution for NormalDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        todo!()
    }
}
