use crate::prdistribution::PRDistribution;

pub struct TriangleDistribution {
    p1: f32,
    p2: f32,
}

impl TriangleDistribution {
    pub fn new(p1: f32, p2: f32) -> Self {
        TriangleDistribution { p1, p2 }
    }
}

impl PRDistribution for TriangleDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32> {
        xs.iter()
            .map(|x| self.p1 + *x as f32 / m as f32 * self.p2)
            .collect()
    }
}
