pub const MOD: u32 = 1024;

pub trait PRGenerator {
    fn next(&mut self) -> u32;
}
