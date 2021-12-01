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

    let sum_sliding_windows = sum_three_measurement_windows(&measurements);
    println!(
        "part 2: {}",
        count_number_of_increases(&sum_sliding_windows)
    );

    Ok(())
}

fn count_number_of_increases(measurements: &[u16]) -> usize {
    measurements.iter().windows().filter(|[a, b]| b > a).count()
}

fn sum_three_measurement_windows(measurements: &[u16]) -> Vec<u16> {
    measurements
        .iter()
        .windows()
        .map(|[x, y, z]| x + y + z)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_passes_aoc_testcase_part1() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, count_number_of_increases(&measurements));
    }

    #[test]
    fn it_finds_sum_of_three_measurement_windows() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(
            vec![607, 618, 618, 617, 647, 716, 769, 792],
            sum_three_measurement_windows(&measurements)
        );
    }

    #[test]
    fn it_passes_aoc_testcase_part2() {
        let measurements = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let windowed_sums = sum_three_measurement_windows(&measurements);
        assert_eq!(5, count_number_of_increases(&windowed_sums));
    }
}
