use std::error;

use aoc2021::{get_inputs, config};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = config::Config::new()?;
    let file_contents = get_inputs::as_string(&config.day)?;

    let data: Vec<i32> = file_contents.lines()
        .map(|line| line.parse::<i32>().unwrap()).collect();

    run(data, config.part);

    Ok(())
}

fn run(data: Vec<i32>, part: u8) {
    match part {
        1 => solve1(data),
        _ => solve2(data),
    }

}

fn solve1(data: Vec<i32>) {
    let mut result = 0;
    for i in 1..data.len() {
        if data[i] > data[i - 1] {
            result += 1;
        }
    }
    println!("{}", result);
}

fn solve2(data: Vec<i32>) {
    let mut result = 0;
    for i in 3..data.len() {
        if data[i-3] + data[i - 2] + data[i - 1] < data[i - 2] + data[i - 1] + data[i] {
            result += 1;
        }
    }
    println!("{}", result);
}