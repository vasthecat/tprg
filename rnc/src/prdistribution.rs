pub trait PRDistribution {
    fn distribute_numbers(&self, m: u32, xs: &[u32]) -> Vec<f32>;
}
