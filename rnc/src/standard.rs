use crate::prdistribution::PRDistribution;

pub struct StandardDistribution {
    p1: f32,
    p2: f32,
}

impl StandardDistribution {
    pub fn new(p1: f32, p2: f32) -> Self {
        StandardDistribution { p1, p2 }
    }
}

impl PRDistribution for StandardDistribution {
    fn distribute_numbers(&self, xs: &Vec<u32>) -> Vec<f32> {
        xs.into_iter().map(|x| *x as f32 / 1024.0).collect()
    }
}
