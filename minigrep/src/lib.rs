use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::process;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[derive(Debug)]
struct SelfDefineError {}

impl std::fmt::Display for SelfDefineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "xxx")
    }
}

impl std::error::Error for SelfDefineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// dyn for dynamic
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // let contents = fs::read_to_string(config.filename).expect("read file error.");
    let contents = fs::read_to_string(config.filename)?;
    println!("content: {}", contents);
    Ok(())
    //  Err(Box::new(SelfDefineError {}))
}
