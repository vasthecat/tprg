use std::fs::File;
use std::io::Write;

mod clp;
use crate::clp::{parse_args, Config, DistributionType};

mod prdistribution;
use crate::prdistribution::PRDistribution;

mod binomial;
mod exponential;
mod gamma;
mod logistic;
mod lognormal;
mod normal;
mod standard;
mod triangle;
use crate::binomial::BinomialDistribution;
use crate::exponential::ExponentialDistribution;
use crate::gamma::GammaDistribution;
use crate::logistic::LogisticDistribution;
use crate::lognormal::LognormalDistribution;
use crate::normal::NormalDistribution;
use crate::standard::StandardDistribution;
use crate::triangle::TriangleDistribution;

fn main() {
    match parse_args(std::env::args()) {
        Ok(conf) => match construct_distribution(&conf) {
            Ok(distr) => {
                let numbers = read_numbers(&conf);
                let module = numbers[0];
                let numbers = Vec::from(&numbers[1..]);
                let distributed = distr.distribute_numbers(module, &numbers);
                write_numbers(&conf, &distributed);
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
        DistributionType::Standrard => {
            Ok(Box::new(StandardDistribution::new(conf.p1, conf.p2)))
        }
        DistributionType::Triangle => {
            Ok(Box::new(TriangleDistribution::new(conf.p1, conf.p2)))
        }
        DistributionType::Exponential => {
            Ok(Box::new(ExponentialDistribution::new(conf.p1, conf.p2)))
        }
        DistributionType::Normal => {
            Ok(Box::new(NormalDistribution::new(conf.p1, conf.p2)))
        }
        DistributionType::Gamma => {
            match conf.p3 {
                Some(p3) => {
                    Ok(Box::new(GammaDistribution::new(conf.p1, conf.p2, p3)))
                }
                None => Err("Параметр p3 обязателен для гамма-распределения"
                    .to_string()),
            }
        }
        DistributionType::Lognormal => {
            Ok(Box::new(LognormalDistribution::new(conf.p1, conf.p2)))
        }
        DistributionType::Logistic => {
            Ok(Box::new(LogisticDistribution::new(conf.p1, conf.p2)))
        }
        DistributionType::Binomial => {
            Ok(Box::new(BinomialDistribution::new(conf.p1, conf.p2)))
        }
    }
}

fn read_numbers(conf: &Config) -> Vec<u32> {
    std::fs::read_to_string(&conf.file)
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}

fn write_numbers(conf: &Config, numbers: &Vec<f32>) {
    let file_name = format!("distr-{}.dat", conf.distribution.to_string());
    let mut file = File::create(file_name).unwrap();

    for i in 0..numbers.len() {
        let num = numbers[i];
        if i == numbers.len() - 1 {
            let _ = file.write_all(format!("{}", num).as_bytes());
        } else {
            let _ = file.write_all(format!("{},", num).as_bytes());
        }
    }
}
