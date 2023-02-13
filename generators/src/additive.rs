use crate::prgenerator::PRGenerator;

pub struct AdditivePRG {
    vec: Vec<u64>,
}

impl AdditivePRG {
    pub fn new(vec: &Vec<u64>) -> Self {
        Self { vec: vec.clone() }
    }
}

impl PRGenerator for AdditivePRG {
    fn next(&mut self) -> u64 {
        return 0;
    }
}
