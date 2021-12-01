use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Result;
use itermore::IterMore;

fn main() -> Result<()> {
    let puzzle_input = File::open("input.txt")?;
    let measurements: Vec<u16> = BufReader::new(puzzle_input)
        .lines()
        .map(|m| m.unwrap().parse::<u16>().unwrap())
        .collect();

    println!("part 1: {}", count_number_of_increases(&measurements));
    Ok(())
}

fn count_number_of_increases(measurements: &[u16]) -> usize {
    measurements.iter().windows().filter(|[a, b]| b > a).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_passes_aoc_testcase_part1() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, count_number_of_increases(&measurements));
    }
}
