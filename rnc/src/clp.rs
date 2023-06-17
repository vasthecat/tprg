#[derive(Debug, Clone, Copy)]
pub enum DistributionType {
    Standrard,   // стандартное равномерное с заданным интервалом;
    Triangle,    // треугольное распределение;
    Exponential, // общее экспоненциальное распределение;
    Normal,      // нормальное распределение;
    Gamma,       // гамма распределение;
    Lognormal,   // логнормальное распределение;
    Logistic,    // логистическое распределение;
    Binomial,    // биномиальное распределение.
}

impl DistributionType {
    fn parse(name: &str) -> Option<Self> {
        match name {
            "st" => Some(DistributionType::Standrard),
            "tr" => Some(DistributionType::Triangle),
            "ex" => Some(DistributionType::Exponential),
            "nr" => Some(DistributionType::Normal),
            "gm" => Some(DistributionType::Gamma),
            "ln" => Some(DistributionType::Lognormal),
            "ls" => Some(DistributionType::Logistic),
            "bi" => Some(DistributionType::Binomial),
            &_ => None,
        }
    }
}

impl ToString for DistributionType {
    fn to_string(&self) -> String {
        match self {
            DistributionType::Standrard => "st",
            DistributionType::Triangle => "tr",
            DistributionType::Exponential => "ex",
            DistributionType::Normal => "nr",
            DistributionType::Gamma => "gm",
            DistributionType::Lognormal => "ln",
            DistributionType::Logistic => "ls",
            DistributionType::Binomial => "bi",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct Config {
    pub distribution: DistributionType,
    pub file: String,
    pub p1: f32,
    pub p2: f32,
    pub p3: Option<f32>,
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

const HELP_STR: &str =
    "Использование: ./rnc /d:<код> /f:<имя_файла> /p1:<число> /p2:<число> [/p3:<число>] [/h]
/d:<код_метода>       - параметр указывает на метод распределения ППСЧ
  Допустимые значения:
   * st – стандартное равномерное с заданным интервалом;
   * tr – треугольное распределение;
   * ex – общее экспоненциальное распределение;
   * nr – нормальное распределение;
   * gm – гамма распределение;
   * ln – логнормальное распределение;
   * ls – логистическое распределение;
   * bi – биномиальное распределение.
/f:<полное_имя_файла> - полное имя файла, из которого будет считываться последовательность.
/p1:<число>           - первый параметр.
/p2:<число>           - второй параметр.
/p3:<число>           - третий параметр (для гамма-распределения).
/h                    - вывод доступных параметров";

pub fn parse_args(args_iter: std::env::Args) -> Result<Config, String> {
    let mut distribution = None;
    let mut file = String::from("rnd.dat");
    let mut p1 = None;
    let mut p2 = None;
    let mut p3 = None;

    for arg in args_iter {
        if !arg.starts_with('/') {
            continue;
        }

        let rest = arg[1..].to_string();
        let (name, rest) = parse_name(&rest);

        if name == "h" {
            return Err(HELP_STR.to_string());
        }

        if !rest.starts_with(':') {
            return Err(format!(
                "Не указано значение параметра '{}'\n\n{}",
                name, HELP_STR
            ));
        }

        let value = rest[1..].to_string();
        match name.as_str() {
            "d" => match DistributionType::parse(&value) {
                None => {
                    return Err(format!(
                        "Недопустимое значение кода распределения\n\n{}",
                        HELP_STR
                    ))
                }
                distrtype => distribution = distrtype,
            },
            "f" => file = value,
            "p1" => match value.parse::<f32>() {
                Ok(num) => p1 = Some(num),
                Err(_) => {
                    return Err("Значение аргумента p1 должно быть \
                        неотрицательным числом"
                        .to_string())
                }
            },
            "p2" => match value.parse::<f32>() {
                Ok(num) => p2 = Some(num),
                Err(_) => {
                    return Err("Значение аргумента p2 должно быть \
                        неотрицательным числом"
                        .to_string())
                }
            },
            "p3" => match value.parse::<f32>() {
                Ok(num) => p3 = Some(num),
                Err(_) => {
                    return Err("Значение аргумента p3 должно быть \
                        неотрицательным числом"
                        .to_string())
                }
            },
            argname => {
                return Err(format!(
                    "Неизвестный параметр '{}'\n\n{}",
                    argname, HELP_STR
                ))
            }
        }
    }

    if distribution.is_none() {
        return Err(format!(
            "Параметр 'd' является обязательным\n\n{}",
            HELP_STR
        ));
    }

    if p1.is_none() {
        return Err(format!(
            "Параметр 'p1' является обязательным\n\n{}",
            HELP_STR
        ));
    }

    if p2.is_none() {
        return Err(format!(
            "Параметр 'p2' является обязательным\n\n{}",
            HELP_STR
        ));
    }

    if let (Some(distribution), Some(p1), Some(p2)) = (distribution, p1, p2) {
        Ok(Config {
            distribution,
            file,
            p1,
            p2,
            p3,
        })
    } else {
        Err(HELP_STR.to_string())
    }
}
