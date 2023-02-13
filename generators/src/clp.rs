#[derive(Debug)]
pub enum GeneratorType {
    LCG,             // линейный конгруэнтный метод;
    Additive,        // аддитивный метод;
    FiveParam,       // пятипараметрический метод;
    LFSR,            // регистр сдвига с обратной связью (РСЛОС);
    NFSR,            // нелинейная комбинация РСЛОС;
    MersenneTwister, // вихрь Мерсенна;
    RC4,             // RC4;
    RSA,             // ГПСЧ на основе RSA;
    BBS,             // алгоритм Блюма-Блюма-Шуба;
}

impl GeneratorType {
    fn parse(name: &String) -> Option<Self> {
        match name.as_str() {
            "lc" => Some(GeneratorType::LCG),
            "add" => Some(GeneratorType::Additive),
            "5p" => Some(GeneratorType::FiveParam),
            "lfsr" => Some(GeneratorType::LFSR),
            "nfsr" => Some(GeneratorType::NFSR),
            "mt" => Some(GeneratorType::MersenneTwister),
            "rc4" => Some(GeneratorType::RC4),
            "rsa" => Some(GeneratorType::RSA),
            "bbs" => Some(GeneratorType::BBS),
            &_ => None,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub generator: GeneratorType,
    pub init: Vec<u64>,
    pub n: u64,
    pub file: String,
}

fn parse_name(arg: &String) -> (String, String) {
    let mut name = Vec::new();
    let mut i = 0;

    while !arg[i..].starts_with(':') {
        name.push(arg.as_bytes()[i]);
        i += 1;
        if i >= arg.len() {
            break;
        }
    }
    let name = String::from_utf8(name).unwrap();
    return (name, arg[i..].to_string());
}

const HELP_STR: &'static str =
    "Использование: ./generators /g:<код> /i:<число> [/n:<длина>] [/f:<имя_файла>] [/h]
/g:<код_метода>       - параметр указывает на метод генерации ПСЧ
  Допустимые значения:
  * lc   - линейный конгруэнтный метод;
  * add  - аддитивный метод;
  * 5p   - пятипараметрический метод;
  * lfsr - регистр сдвига с обратной связью (РСЛОС);
  * nfsr - нелинейная комбинация РСЛОС;
  * mt   - вихрь Мерсенна;
  * rc4  - RC4;
  * rsa  - ГПСЧ на основе RSA;
  * bbs  - алгоритм Блюма-Блюма-Шуба;
/i:<число>            - инициализационный вектор генератора.
/n:<длина>            - количество генерируемых чисел
/f:<полное_имя_файла> - полное имя файла, в который будут выводиться данные.
/h                    - Вывод доступных параметров";

pub fn parse_args(args_iter: std::env::Args) -> Result<Config, &'static str> {
    let mut generator = None;
    let mut init = None;
    let mut n: u64 = 10000;
    let mut file = String::from("rnd.dat");

    for arg in args_iter {
        if !arg.starts_with('/') {
            continue;
        }

        let rest = arg[1..].to_string();
        let (name, rest) = parse_name(&rest);

        if name == "h" {
            return Err(HELP_STR);
        }

        if !rest.starts_with(':') {
            return Err(HELP_STR);
        }

        let value = rest[1..].to_string();
        match name.as_str() {
            "g" => match GeneratorType::parse(&value) {
                None => return Err("Недопустимое значение кода генератора"),
                gentype => generator = gentype,
            },
            "n" => match value.parse::<u64>() {
                Ok(num) => n = num,
                Err(_) => return Err("Значение аргумента n должно быть неотрицательным числом"),
            },
            "f" => file = value,
            "i" => {
                let mut vec = Vec::new();
                for num in value.split(',') {
                    match num.parse::<u64>() {
                        Ok(n) => vec.push(n),
                        Err(_) => return Err(
                            "Значения вектора инициализации должны быть неотрицательными числами",
                        ),
                    }
                }
                init = Some(vec);
            }
            &_ => return Err(HELP_STR),
        }
    }

    if let (Some(generator), Some(init)) = (generator, init) {
        return Ok(Config {
            generator,
            init,
            n,
            file,
        });
    }

    return Err(HELP_STR);
}
