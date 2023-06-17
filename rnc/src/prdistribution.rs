pub trait PRDistribution {
    fn distribute_numbers(&self, xs: Vec<u32>) -> Vec<u32>;
}
