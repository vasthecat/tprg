use std::fs::File;
use std::io::Write;

mod clp;
use crate::clp::{parse_args, Config, DistributionType};

mod prdistribution;
use crate::prdistribution::PRDistribution;

fn main() {
    match parse_args(std::env::args()) {
        Ok(conf) => match construct_distribution(&conf) {
            Ok(gen) => {
                // let numbers = generate_numbers(&conf, gen);
                // write_numbers(&conf, &numbers);
            }
            Err(msg) => println!("{}", msg),
        },
        Err(msg) => println!("{}", msg),
    };
}

fn construct_distribution(
    conf: &Config,
) -> Result<Box<dyn PRDistribution>, String> {
    match conf.distribution {
        _ => todo!(),
    }
}

fn read_numbers(conf: &Config) -> Vec<u32> {
    Vec::new()
}

// fn generate_numbers<T: PRGenerator + ?Sized>(
//     conf: &Config,
//     mut gen: Box<T>,
// ) -> Vec<u32> {
//     let mut numbers = Vec::new();
//     let mut pb = ProgressBar::new(conf.n);
//     for _ in 0..conf.n {
//         numbers.push(gen.next());
//         pb.message("Генерация чисел: ");
//         pb.inc();
//     }
//     pb.finish_println("Генерация чисел завершена.");
//     println!();
//     return numbers;
// }

// fn write_numbers(conf: &Config, numbers: &Vec<u32>) {
//     let mut file = File::create(&conf.file).unwrap();
//     let mut pb = ProgressBar::new(numbers.len().try_into().unwrap());
//     for i in 0..numbers.len() {
//         let num = numbers[i];
//         if i == numbers.len() - 1 {
//             let _ = file.write_all(format!("{}", num).as_bytes());
//         } else {
//             let _ = file.write_all(format!("{},", num).as_bytes());
//         }
//         pb.message("Запись чисел в файл: ");
//         pb.inc();
//     }
//     pb.finish_println("Запись чисел завершена.");
//     println!();
// }
