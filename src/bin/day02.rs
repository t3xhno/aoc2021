use std::error;
use itertools::Itertools;

use aoc2021::{input::Input, config::Config};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new()?;
    let file_contents = Input::new(&config.day).as_string()?;
    let data = parse_input(&file_contents);
    println!("{}", solve1(&data));
    println!("{}", solve2(&data));
    Ok(())
}

fn parse_input(input: &str) -> Vec<(u8, i32)> {
    input.split_whitespace()
        .tuples()
        .map(|(op, i)| (op.as_bytes()[0], i.parse::<i32>().unwrap()))
        .collect()
}

fn solve1(data: &Vec<(u8, i32)>) -> i32 {
    let (hor, ver) = data.iter()
        .fold((0, 0), |(hor, ver), (op, i)| match op {
            b'f' => (hor + i, ver),
            b'd' => (hor, ver + i),
            _ => (hor, ver - i),
        });
    hor * ver
}

fn solve2(data: &Vec<(u8, i32)>) -> i32 {
    let (hor, ver, _) = data.iter()
        .fold((0, 0, 0), |(hor, ver, aim), (op, i)| match op {
            b'f' => (hor + i, ver + aim * i, aim),
            b'd' => (hor, ver, aim + i),
            _ => (hor, ver, aim - i),
        });
    hor * ver
}
