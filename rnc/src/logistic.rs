use crate::prdistribution::PRDistribution;

pub struct LogisticDistribution {
    p1: f32,
    p2: f32,
}

impl LogisticDistribution {
    pub fn new(p1: f32, p2: f32) -> Self {
        LogisticDistribution { p1, p2 }
    }
}

impl PRDistribution for LogisticDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        todo!()
    }
}
