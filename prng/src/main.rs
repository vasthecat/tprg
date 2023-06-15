use std::fs::File;
use std::io::Write;

mod clp;
use clp::{parse_args, Config, GeneratorType};

mod prgenerator;
use prgenerator::PRGenerator;

mod additive;
mod fp;
mod lfsr;
mod linear;
use additive::AdditivePRG;
use fp::FiveParamPRG;
use lfsr::LfsrPRG;
use linear::LinearPRG;

// Генератор запускается только если (1) корректно введены все аргументы,
// (2) инициализационный вектор соответствует указанному генератору.
// Иначе, будет выведено сообщение с ошибкой.
fn main() {
    match parse_args(std::env::args()) {
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

// Здесь конструируются генераторы на основе введённых в командной строке
// параметров. Каждый из генераторов реализует признак (интерфейс) PRGenerator,
// поэтому необходимо хранить ссылку на объект с общим интерфейсом для
// использования в других функциях. Если на основе указанных параметров
// невозможно сконструировать генератор, то возвращается ошибка.
fn construct_generator(conf: &Config) -> Result<Box<dyn PRGenerator>, String> {
    match conf.generator {
        GeneratorType::Lcg => {
            if conf.init.len() != 4 {
                Err("Для линейного конгруэнтного генератора \
                     инициализационный вектор должен содержать 4 элемента"
                    .to_string())
            } else {
                Ok(Box::new(LinearPRG::new(
                    conf.init[0],
                    conf.init[1],
                    conf.init[2],
                    conf.init[3],
                )))
            }
        }
        GeneratorType::Additive => {
            if conf.init.len() < 3 {
                return Err("Для аддитивного генератора инициализационный \
                           вектор должен содержать m, j и k"
                    .to_string());
            }
            let m = conf.init[0];
            let k = conf.init[1] as usize;
            let j = conf.init[2] as usize;
            let xs = Vec::from(&conf.init[3..]);
            if j <= k || k < 1 {
                return Err("Для аддитивного генератора должно выполняться \
                            j > k >= 1"
                    .to_string());
            }
            if xs.len() < j {
                return Err(
                    "Для аддитивного генератора длина инициализационного \
                            вектора len(xs) должна быть >= j"
                        .to_string(),
                );
            }
            return Ok(Box::new(AdditivePRG::new(m, j, k, xs)));
        }
        GeneratorType::FiveParam => {
            let len = conf.init.len();
            if len < 5 {
                return Err("Для 5-параметрического генератора необходимо \
                           указать параметры p,q1,q2,q3,w"
                    .to_string());
            }
            let p = conf.init[0] as usize;
            let q1 = conf.init[1] as usize;
            let q2 = conf.init[2] as usize;
            let q3 = conf.init[3] as usize;
            let w = conf.init[4];
            let xs = Vec::from(&conf.init[5..]);
            if xs.len() < p as _ {
                return Err(
                    "Для 5-параметрического генератора необходимо указать \
                            p коэффициентов X"
                        .to_string(),
                );
            }
            return Ok(Box::new(FiveParamPRG::new(p, q1, q2, q3, w, xs)));
        }
        GeneratorType::Lfsr => {
            let len = conf.init.len();
            if len % 2 != 0 {
                return Err(
                    "Инициализационный вектор должен содержать чётное \
                            количество элементов"
                        .to_string(),
                );
            }
            let coeff = Vec::from(&conf.init[..len / 2]);
            let init = Vec::from(&conf.init[len / 2..]);
            return Ok(Box::new(LfsrPRG::new(coeff, init)));
        }
        _ => todo!(),
    }
}

// Так как все генераторы имеют общий интерфейс, то просто генерируем заданное
// количество чисел по одному с помощью метода .next()
fn generate_numbers<T: PRGenerator + ?Sized>(
    conf: &Config,
    mut gen: Box<T>,
) -> Vec<u32> {
    let mut numbers = Vec::new();
    for _ in 0..conf.n {
        numbers.push(gen.next());
    }
    return numbers;
}

fn write_numbers(conf: &Config, numbers: &Vec<u32>) {
    let mut file = File::create(&conf.file).unwrap();
    for i in 0..numbers.len() {
        let num = numbers[i];
        if i == numbers.len() - 1 {
            let _ = file.write_all(format!("{}", num).as_bytes());
        } else {
            let _ = file.write_all(format!("{},", num).as_bytes());
        }
    }
}
