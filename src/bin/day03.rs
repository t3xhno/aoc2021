use std::error;

use aoc2021::{input::Input, config::Config};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::new()?;
    let file_contents = Input::new(&config.day).as_string()?;
    let data = parse_input(&file_contents);
    println!("{}", solve1(&data));
    Ok(())
}

fn max_bit(nums: &[u32], bit: usize) -> u32 {
    let mut c = [0, 0];
    for &x in nums {
        c[(x as usize >> bit) & 1] += 1;
    }
    (c[1] >= c[0]) as u32
}

fn parse_input(input: &str) -> Vec<u32> {
    input.lines()
         .map(|line| u32::from_str_radix(line, 2).unwrap())
         .collect()
}

fn solve1(data: &[u32]) -> u32 {
    let x = (0..12).map(|i| max_bit(data, i) << i)
        .sum::<u32>();
    x * (!x & 0xfff)
}
