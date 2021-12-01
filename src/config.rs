use std::env;

pub struct Config {
    pub day: String,
    pub part: u8,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args = env::args().collect::<Vec<String>>();
        match args.len() > 2 {
            false => Err("Needs 2 arguments [day0x + part]"),
            _ => Ok(Config {
                day: args[1].clone(),
                part: match args[2].parse().unwrap() {
                    p if p > 2 => return Err("There are only 2 parts [1 and 2]"),
                    p => p,
                },
            }),
        }
    }
}