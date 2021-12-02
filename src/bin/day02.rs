use std::error;

use aoc2021::{input::Input, config::Config};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new()?;
    let file_contents = Input::new(&config.day).as_string()?;
    let data: Vec<(&str, i32)> = file_contents.lines().map(input_mapper).collect();
    println!("{}", solve1(&data));
    println!("{}", solve2(&data));
    Ok(())
}

fn input_mapper(line: &str) -> (&str, i32) {
    let mut tuple: (&str, i32) = ("", 0);
    let split = line.trim().split(" ").collect::<Vec<&str>>();
    tuple.0 = split[0]; tuple.1 = split[1].parse::<i32>().unwrap();
    tuple
}

fn solve1(data: &Vec<(&str, i32)>) -> i32 {
    let mut depth = 0; let mut dist = 0;
    data.into_iter().for_each(|op| match op {
        ("up", x) => depth -= x,
        ("down", x) => depth += x,
        (_, x) => dist += x,
    });
    depth * dist
}

fn solve2(data: &Vec<(&str, i32)>) -> i32 {
    let mut depth = 0; let mut dist = 0; let mut aim = 0;
    data.into_iter().for_each(|op| match op {
        ("down", x) => aim += x,
        ("up", x) => aim -= x,
        (_, x) => {
            dist += x; depth += x * aim;
        }
    });
    depth * dist
}
