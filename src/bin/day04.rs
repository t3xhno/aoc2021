use aoc2021::{config::Config, input::Input};
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, module 4 :)");
    let config = Config::new()?;
    let file_contents = Input::new(&config.day).as_string()?;
    Ok(())
}
