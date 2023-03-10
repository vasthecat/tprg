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
        GeneratorType::LCG => {
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
            if conf.init.len() != 3 {
                return Err("Для аддитивного генератора инициализационный \
                           вектор должен содержать 3 элемента"
                    .to_string());
            }
            let m = conf.init[0];
            let j = conf.init[1];
            let k = conf.init[2];
            if j <= k || k < 1 {
                return Err("Для аддитивного генератора должно выполняться \
                            j > k >= 1"
                    .to_string());
            }
            return Ok(Box::new(AdditivePRG::new(m, j, k)));
        }
        _ => todo!(),
    }
}

// Так как все генераторы имеют общий интерфейс, то просто генерируем заданное
// количество чисел по одному с помощью метода .next()
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
