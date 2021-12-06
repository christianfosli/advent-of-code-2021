use std::collections::HashMap;

fn count_lanternfish_after_n_days(fish: &[u8], n: usize) -> usize {
    // store as map < internal_timer: u8 , number_of_fish: usize >
    let mut fishmap: HashMap<u8, usize> = (0..=8)
        .map(|timer| (timer, fish.iter().filter(|&&f| f == timer).count()))
        .collect();

    for _ in 0..n {
        let zeros = fishmap[&0];

        // fish with timer != 0 decrease their timer by one
        for (timer, &count) in fishmap.clone().iter().filter(|(&timer, _)| timer != 0) {
            *fishmap.get_mut(&(timer - 1)).unwrap() = count;
        }

        // fish with timer 0 reset to 6 and create babies with timer 8
        *fishmap.get_mut(&8).unwrap() = zeros;
        *fishmap.get_mut(&6).unwrap() += zeros;
    }

    fishmap.iter().fold(0, |acc, (_, &count)| acc + count)
}

fn main() -> Result<(), anyhow::Error> {
    let lanternfish: Vec<u8> = std::fs::read_to_string("input.txt")?
        .trim_end()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    println!(
        "part 1: {}",
        count_lanternfish_after_n_days(&lanternfish, 80)
    );

    println!(
        "part 2: {}",
        count_lanternfish_after_n_days(&lanternfish, 256)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_passes_aoc_testcase_1() {
        let lanternfish = vec![3, 4, 3, 1, 2];
        assert_eq!(5934, count_lanternfish_after_n_days(&lanternfish, 80));
    }

    #[test]
    fn it_passes_aoc_testcase_2() {
        let lanternfish = vec![3, 4, 3, 1, 2];
        assert_eq!(
            26984457539,
            count_lanternfish_after_n_days(&lanternfish, 256)
        );
    }
}
