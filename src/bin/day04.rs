use aoc2021::{config::Config, input::Input};

use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new()?;
    let file_contents = Input::new(&config.day).as_string()?;
    let data = parase_input(&file_contents);
    println!("{:?}", data);
    Ok(())
}

fn parase_input(_input: &str) {}
