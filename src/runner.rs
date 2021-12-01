pub struct Runner {}

type FF = fn(Vec<i32>) -> usize;

impl Runner {
    pub fn run(data: Vec<i32>, part: u8, solve1: FF, solve2: FF) -> usize
    {
        match part {
            1 => solve1(data),
            _ => solve2(data),
        }
    }
}