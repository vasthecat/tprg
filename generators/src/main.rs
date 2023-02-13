use std::fs::File;
use std::io::Write;

mod clp;
use clp::{parse_args, Config, GeneratorType};

mod prgenerator;
use prgenerator::PRGenerator;

mod additive;
mod linear;
use additive::AdditivePRG;
use linear::LinearPRG;

fn generate_numbers<T: PRGenerator + ?Sized>(conf: &Config, mut gen: Box<T>) -> Vec<u64> {
    let mut numbers = Vec::new();
    for _ in 0..conf.n {
        numbers.push(gen.next());
    }
    return numbers;
}

fn write_numbers(conf: &Config, numbers: &Vec<u64>) {
    let mut file = File::create(&conf.file).unwrap();
    for num in numbers {
        let _ = file.write_all(format!("{}\n", num).as_bytes());
    }
}

fn construct_generator(conf: &Config) -> Result<Box<dyn PRGenerator>, &'static str> {
    match conf.generator {
        GeneratorType::LCG => {
            if conf.init.len() < 4 {
                Err("Для линейного конгруэнтного генератора \
                     инициализационный вектор должен содержать 4 элемента")
            } else {
                Ok(Box::new(LinearPRG::new(
                    conf.init[0],
                    conf.init[1],
                    conf.init[2],
                    conf.init[3],
                )))
            }
        }
        GeneratorType::Additive => Ok(Box::new(AdditivePRG::new(&conf.init))),
        _ => todo!(),
    }
}

fn main() {
    let config = parse_args(std::env::args());
    match config {
        Ok(conf) => match construct_generator(&conf) {
            Ok(gen) => {
                let numbers = generate_numbers(&conf, gen);
                write_numbers(&conf, &numbers);
            }
            Err(msg) => println!("{}", msg),
        },
        Err(msg) => println!("{}", msg),
    };
}
