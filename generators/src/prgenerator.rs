pub trait PRGenerator {
    fn next(&mut self) -> u64;
}
