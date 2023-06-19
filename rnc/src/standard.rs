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
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        xs.iter()
            .map(|x| self.p1 + *x as f32 / m as f32 * self.p2)
            .collect()
    }
}
