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
        todo!()
    }
}
