use std::error;

use aoc2021::{config::Config, input::Input, runner::Runner};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new()?;
    let file_contents = Input::new(&config.day).as_string()?;
    let data: Vec<i32> = file_contents.lines().map(input_mapper).collect();
    println!("{}", Runner::run(data, config.part, solve1, solve2));
    Ok(())
}

fn input_mapper(line: &str) -> i32 {
    line.parse::<i32>().unwrap()
}

fn solve1(data: Vec<i32>) -> usize {
    data.windows(2).filter(|&line| line[1] > line[0]).count()
}

fn solve2(data: Vec<i32>) -> usize {
    data.windows(4)
        .filter(|&quartet| quartet[3] > quartet[0])
        .count()
}
