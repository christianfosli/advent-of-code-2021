use ndarray::prelude::*;

fn count_flashes(octopuses: &str, steps: usize) -> Result<usize, anyhow::Error> {
    let mut matrix = Array::from_shape_vec(
        (10, 10),
        octopuses
            .replace('\n', "")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect(),
    )?;

    let mut flashes = 0;

    for _ in 0..steps {
        // first the energy level of each octopus incrases by 1
        matrix = matrix + Array::<u8, _>::ones((10, 10));

        // then any octopus with a energy level > 9 flashes
        let mut about_to_flash = Vec::new();
        for row in 0..10 {
            for col in 0..10 {
                if matrix[[row, col]] > 9 {
                    about_to_flash.push([row, col]);
                }
            }
        }
        flashes += about_to_flash.len();

        // finally any octopus that flashes has its energy level reset to 0
        for pos in about_to_flash {
            *matrix.get_mut(pos).unwrap() = 0;
        }
    }

    Ok(flashes)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const OCTOPUSES: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn it_passes_aoc_testcase_1() {
        assert_eq!(1656, count_flashes(&OCTOPUSES, 10).unwrap());
    }
}
