use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::ParseIntError,
};

use itermore::IterMore;

fn main() -> Result<(), anyhow::Error> {
    let puzzle_input = File::open("input.txt")?;

    let measurements: Vec<u16> = BufReader::new(puzzle_input)
        .lines()
        .collect::<Result<Vec<String>, io::Error>>()?
        .into_iter()
        .map(|measurement| measurement.parse())
        .collect::<Result<Vec<u16>, ParseIntError>>()?;

    println!("part 1: {}", count_number_of_increases(&measurements));

    let sliding_windows_sums = sum_three_measurement_windows(&measurements);
    let number_of_increases_2 = count_number_of_increases(&sliding_windows_sums);
    println!("part 2: {}", number_of_increases_2);

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
