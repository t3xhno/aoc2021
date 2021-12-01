use std::error;

use aoc2021::{get_inputs, config::Config};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new()?;
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
    println!("{}", data.windows(2)
        .filter(|&line| line[1] > line[0])
        .count());
}

fn solve2(data: Vec<i32>) {
    println!("{}", data.windows(4)
        .filter(|quartet| quartet[3] > quartet[0])
        .count());
}