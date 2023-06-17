use crate::prdistribution::PRDistribution;

pub struct StandardDistribution {}

impl StandardDistribution {
    pub fn new() -> Self {
        StandardDistribution {}
    }

    pub fn distribute_number(x: u32, m: u32) -> f32 {
        x as f32 / m as f32
    }
}

impl PRDistribution for StandardDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        xs.iter()
            .map(|x| StandardDistribution::distribute_number(*x, m))
            .collect()
    }
}
