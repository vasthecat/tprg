mod clp;
use clp::{parse_args, Config, GeneratorType};

fn main() {
    let config = parse_args(std::env::args());
    match config {
        Ok(conf) => println!("{:?}", conf),
        Err(msg) => println!("{}", msg),
    }
}
