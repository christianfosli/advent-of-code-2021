/// part 1
fn count_lanternfish_after_n_days(lanternfish: &[u8], n: usize) -> usize {
    (0..n)
        .fold(lanternfish.to_vec(), |fish, _day| {
            let num_newborns = fish.iter().filter(|&&f| f == 0).count();
            let new_fish = fish
                .iter()
                .map(|&f| if f == 0 { 6 } else { f - 1 })
                .chain(vec![8; num_newborns]);
            new_fish.collect::<Vec<u8>>()
        })
        .iter()
        .count()
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
    #[ignore = "too slow!"]
    fn it_passes_aoc_testcase_2() {
        let lanternfish = vec![3, 4, 3, 1, 2];
        assert_eq!(
            26984457539,
            count_lanternfish_after_n_days(&lanternfish, 256)
        );
    }
}
