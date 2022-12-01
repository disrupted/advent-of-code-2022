use std::error::Error;

use emergence::AoC;

fn main() -> Result<(), Box<dyn Error>> {
    let aoc = AoC::new(2022)?;
    let input = aoc.read_or_fetch(1)?;
    let result = solve_part1(&input).unwrap();
    println!("{}:{}", 1 + result.0, result.1);

    let result = solve_part2(&input);
    println!("{}", result);
    Ok(())
}

fn solve_part1(input: &str) -> Option<(usize, u32)> {
    input
        .split("\n\n") // split on empty lines
        .map(|chunk| {
            chunk
                .lines()
                .map(|l| l.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
}

fn solve_part2(input: &str) -> u32 {
    let mut calories = input
        .split("\n\n") // split on empty lines
        .map(|chunk| {
            chunk
                .lines()
                .map(|l| l.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    calories.sort();
    calories.reverse();
    calories.iter().take(3).sum()
}
