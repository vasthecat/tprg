#[derive(Debug, Clone, Copy)]
pub enum GeneratorType {
    Lcg,             // линейный конгруэнтный метод;
    Additive,        // аддитивный метод;
    FiveParam,       // пятипараметрический метод;
    Lfsr,            // регистр сдвига с обратной связью (РСЛОС);
    Nfsr,            // нелинейная комбинация РСЛОС;
    MersenneTwister, // вихрь Мерсенна;
    Rc4,             // RC4;
    Rsa,             // ГПСЧ на основе RSA;
    Bbs,             // алгоритм Блюма-Блюма-Шуба;
}

impl GeneratorType {
    fn parse(name: &str) -> Option<Self> {
        match name {
            "lc" => Some(GeneratorType::Lcg),
            "add" => Some(GeneratorType::Additive),
            "5p" => Some(GeneratorType::FiveParam),
            "lfsr" => Some(GeneratorType::Lfsr),
            "nfsr" => Some(GeneratorType::Nfsr),
            "mt" => Some(GeneratorType::MersenneTwister),
            "rc4" => Some(GeneratorType::Rc4),
            "rsa" => Some(GeneratorType::Rsa),
            "bbs" => Some(GeneratorType::Bbs),
            &_ => None,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub generator: GeneratorType,
    pub init: Vec<u32>,
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
    (name, arg[i..].to_string())
}


const HELP_USAGE: &str =
    "Использование: ./generators /g:<код> /i:<число> [/n:<длина>] [/f:<имя_файла>] [/h]";

const HELP_PARAMS: &str =
"/i:<число>            - инициализационный вектор генератора.
/n:<длина>            - количество генерируемых чисел
/f:<полное_имя_файла> - полное имя файла, в который будут выводиться данные.
/h                    - Вывод доступных параметров";

const HELP_GEN: &str =
    "/g:<код_метода>       - параметр указывает на метод генерации ПСЧ
  Допустимые значения:
  * lc   - линейный конгруэнтный метод; (/i:m,a,c,x0)
  * add  - аддитивный метод; (/i:m,k,j,x0,...,xn), n >= j
  * 5p   - пятипараметрический метод (/i:p,q1,q2,q3,w,x1,...,xp);
  * lfsr - регистр сдвига с обратной связью (РСЛОС); (/i:a0,...,ap,x0,...,xp)
  * nfsr - нелинейная комбинация РСЛОС (/i:a0,...an,b0,...,bn,c0,...cn,w,x1,x2,x3);
  * mt   - вихрь Мерсенна (/i:m,x);
  * rc4  - RC4 (/i:x0,...,x256);
  * rsa  - ГПСЧ на основе RSA (/i:n,e,w,x);
  * bbs  - алгоритм Блюма-Блюма-Шуба (/i:x);";

const HELP_LC: &str =
    "/g:lc - линейный конгруэнтный метод; (/i:m,a,c,x0)
  Параметры:
  * m  - модуль
  * a  - множитель
  * c  - приращение
  * x0 - начальное значение (десятичное число)";

const HELP_ADD: &str =
"/g:add - аддитивный метод; (/i:m,k,j,x0,...,xn), n >= j
  Параметры:
  * m - модуль
  * k - младший индекс
  * j - старший индекс
  * x0,...,xn - n начальных значений (десятичные числа), n >= j";

const HELP_5P: &str =
"/g:5p - пятипараметрический метод (/i:p,q1,q2,q3,w,x1,...,xp);
  Параметры:
  * p - количество элементов
  * q1 - первый индекс
  * q2 - второй индекс
  * q3 - третий индекс
  * w - длина слова в битах
  * x1,...xp - начальное значение регистра (последовательность 0 и 1)";

const HELP_LFSR: &str =
"/g:lfsr - регистр сдвига с обратной связью (РСЛОС); (/i:a0,...,ap,x0,...,xp)
  Параметры:
  * a0,...,ap - вектор коэффициентов (последовательность 0 и 1)
  * x0,...,xp - начальное значение регистра (последовательность 0 и 1)";

const HELP_NFSR: &str =
"/g:nfsr - нелинейная комбинация РСЛОС (/i:a0,...an,b0,...,bn,c0,...cn,w,x1,x2,x3);
  Параметры:
  * a0,...,an - коэффициенты первого РСЛОС (последовательность 0 и 1)
  * b0,...,bn - коэффициенты второго РСЛОС (последовательность 0 и 1)
  * c0,...,cn - коэффициенты третьего РСЛОС (последовательность 0 и 1)
  * w - длина слова в битах
  * x1 - начальное состояние первого РСЛОС (десятичное число)
  * x2 - начальное состояние второго РСЛОС (десятичное число)
  * x3 - начальное состояние третьего РСЛОС (десятичное число)";

const HELP_MT: &str = "/g:mt - вихрь Мерсенна (/i:m,x);
  Параметры:
  * m - модуль
  * x - начальное значение (десятичное число)";

const HELP_RC4: &str = "/g:rc4 - RC4 (/i:x0,...,x256);
  Параметры:
  * x0,...,x256 - начальные значения (256 десятичных чисел)";

const HELP_RSA: &str = "/g:rsa - ГПСЧ на основе RSA (/i:n,e,w,x);
  Параметры:
  * n - модуль n=pq, где p,q - простые числа
  * e - такое число, что 1 < e < (p-1)(q-1), gcd(e, (p-1)(q-1)) = 1
  * x - число из интервала [1,n]
  * w - длина слова в битах";

const HELP_BBS: &str = "/g:bbs - алгоритм Блюма-Блюма-Шуба (/i:x);
  Параметры:
  * x - начальное значение (десятичное число), gcd(x, 16637) = 1";

fn help_string(generator: Option<GeneratorType>) -> String {
    match generator {
        None => format!("{}\n{}\n{}", HELP_USAGE, HELP_GEN, HELP_PARAMS),
        Some(GeneratorType::Lcg) =>
            format!("{}\n{}\n{}", HELP_USAGE, HELP_LC, HELP_PARAMS),
        Some(GeneratorType::Additive) =>
            format!("{}\n{}\n{}", HELP_USAGE, HELP_ADD, HELP_PARAMS),
        Some(GeneratorType::FiveParam) =>
            format!("{}\n{}\n{}", HELP_USAGE, HELP_5P, HELP_PARAMS),
        Some(GeneratorType::Lfsr) =>
            format!("{}\n{}\n{}", HELP_USAGE, HELP_LFSR, HELP_PARAMS),
        Some(GeneratorType::Nfsr) =>
            format!("{}\n{}\n{}", HELP_USAGE, HELP_NFSR, HELP_PARAMS),
        Some(GeneratorType::MersenneTwister) =>
            format!("{}\n{}\n{}", HELP_USAGE, HELP_MT, HELP_PARAMS),
        Some(GeneratorType::Rc4) =>
            format!("{}\n{}\n{}", HELP_USAGE, HELP_RC4, HELP_PARAMS),
        Some(GeneratorType::Rsa) =>
            format!("{}\n{}\n{}", HELP_USAGE, HELP_RSA, HELP_PARAMS),
        Some(GeneratorType::Bbs) =>
            format!("{}\n{}\n{}", HELP_USAGE, HELP_BBS, HELP_PARAMS),
    }
}

pub fn parse_args(args_iter: std::env::Args) -> Result<Config, String> {
    let mut generator = None;
    let mut init = None;
    let mut n: u64 = 10000;
    let mut file = String::from("rnd.dat");

    // Первый аргумент содержит путь до исполняемого файла, поэтому пропускаем
    for arg in args_iter.skip(1) {
        if !arg.starts_with('/') {
            continue;
        }

        let rest = arg[1..].to_string();
        let (name, rest) = parse_name(&rest);

        if name == "h" {
            return Err(help_string(generator));
        }

        if !rest.starts_with(':') {
            return Err(format!(
                "Не указано значение параметра '{}'\n\n{}",
                name, help_string(generator)
            ));
        }

        let value = rest[1..].to_string();
        match name.as_str() {
            "g" => match GeneratorType::parse(&value) {
                None => {
                    return Err(format!(
                        "Недопустимое значение кода генератора\n\n{}",
                        help_string(generator)
                    ))
                }
                gentype => generator = gentype,
            },
            "n" => match value.parse::<u64>() {
                Ok(num) => n = num,
                Err(_) => {
                    return Err(format!(
                        "Значение аргумента n должно быть \
                        неотрицательным числом\n\n{}",
                        help_string(generator)
                    ))
                }
            },
            "f" => file = value,
            "i" => {
                let mut vec = Vec::new();
                for num in value.split(',') {
                    match num.parse::<u32>() {
                        Ok(n) => vec.push(n),
                        Err(_) => {
                            return Err(format!(
                                "Значения вектора инициализации должны \
                                 быть неотрицательными числами\n\n{}",
                                help_string(generator)
                            ))
                        }
                    }
                }
                init = Some(vec);
            }
            argname => {
                return Err(format!(
                    "Неизвестный параметр '{}'\n\n{}",
                    argname, help_string(generator)
                ))
            }
        }
    }

    if generator.is_none() {
        return Err(format!(
            "Параметр 'g' является обязательным\n\n{}",
            help_string(generator)
        ));
    }

    if init.is_none() {
        return Err(format!(
            "Параметр 'i' является обязательным\n\n{}",
            help_string(generator)
        ));
    }

    if let (Some(generator), Some(init)) = (generator, init) {
        Ok(Config {
            generator,
            init,
            n,
            file,
        })
    } else {
        Err(help_string(generator))
    }
}
