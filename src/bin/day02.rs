use itertools::Itertools;
use std::error;

use aoc2021::{config::Config, input::Input, structs::Submarine};

// wjqelqjwwqeqkheqwjjewqljwqe
fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new()?;
    let file_contents = Input::new(&config.day).as_string()?;
    let data = parse_input(&file_contents);
    println!("{}", solve(&data, &mut Submarine::new(), config.part));
    Ok(())
}

fn parse_input(input: &str) -> Vec<(u8, i32)> {
    input
        .split_whitespace()
        .tuples()
        .map(|(op, i)| (op.as_bytes()[1], i.parse::<i32>().unwrap()))
        .collect()
}

fn solve(data: &[(u8, i32)], sub: &mut Submarine, part: u8) -> i32 {
    data.iter().for_each(|(op, i)| match part {
        1 => sub.move_part_1(*op, *i),
        _ => sub.move_part_2(*op, *i),
    });
    sub.dist()
}
