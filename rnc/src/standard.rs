use crate::prdistribution::PRDistribution;

pub struct StandardDistribution {}

impl StandardDistribution {
    pub fn new() -> Self {
        StandardDistribution {}
    }
}

impl PRDistribution for StandardDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        xs.iter().map(|x| *x as f32 / m as f32).collect()
    }
}
