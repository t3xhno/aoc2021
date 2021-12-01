use std::error;

use aoc2021::{input::Input, config::Config};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new()?;
    let file_contents = Input::new(&config.day).as_string()?;
    let data: Vec<i32> = file_contents.lines().map(input_mapper).collect();

    run(data, config.part);

    Ok(())
}

fn input_mapper(line: &str) -> i32 {
    line.parse::<i32>().unwrap()
}

fn run(data: Vec<i32>, part: u8) {
    match part {
        1 => println!("{}", solve1(data)),
        _ => println!("{}", solve2(data)),
    }

}

fn solve1(data: Vec<i32>) -> usize {
    data.windows(2).filter(|&line| line[1] > line[0]).count()
}

fn solve2(data: Vec<i32>) -> usize {
    data.windows(4).filter(|quartet| quartet[3] > quartet[0]).count()
}