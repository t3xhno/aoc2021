use std::error;
use itertools::Itertools;

use aoc2021::{input::Input, config::Config, structs::Submarine};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new()?;
    let file_contents = Input::new(&config.day).as_string()?;
    let data = parse_input(&file_contents);
    println!("{}", solve1(&data, &mut Submarine::new()));
    println!("{}", solve2(&data, &mut Submarine::new()));
    Ok(())
}

fn parse_input(input: &str) -> Vec<(u8, i32)> {
    input.split_whitespace()
        .tuples()
        .map(|(op, i)| (op.as_bytes()[0], i.parse::<i32>().unwrap()))
        .collect()
}

fn solve1(data: &[(u8, i32)], sub: &mut Submarine) -> i32 {
    data.iter().for_each(|(op, i)| sub.move_part_1(*op, *i));
    sub.dist()
}

fn solve2(data: &[(u8, i32)], sub: &mut Submarine) -> i32 {
    data.iter().for_each(|(op, i)| sub.move_part_2(*op, *i));
    sub.dist()
}
