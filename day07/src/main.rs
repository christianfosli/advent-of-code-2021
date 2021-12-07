use std::cmp::{max, min};

/// part 1
fn find_best_alignment_1(crabs: &[u16]) -> Option<(u16, usize)> {
    let max_pos = crabs.iter().max()?;
    let mut best_alignment: Option<(u16, usize)> = None;

    for pos in 0..=*max_pos {
        let req_fuel = crabs.iter().fold(0, |acc, el| {
            acc + usize::from(max(pos, *el) - min(*el, pos))
        });

        if best_alignment == None || req_fuel < best_alignment?.1 {
            best_alignment = Some((pos, req_fuel));
        }
    }

    best_alignment
}

/// part 2
fn find_best_alignment_2(crabs: &[u16]) -> Option<(u16, usize)> {
    let max_pos = crabs.iter().max()?;
    let mut best_alignment: Option<(u16, usize)> = None;

    for pos in 0..=*max_pos {
        let req_fuel = crabs.iter().fold(0, |acc, el| {
            let diff = usize::from(max(pos, *el) - min(*el, pos));
            if diff == 0 || diff == 1 {
                acc + diff
            } else {
                acc + (1..=diff).reduce(|acc, el| acc + el).unwrap()
            }
        });

        if best_alignment == None || req_fuel < best_alignment?.1 {
            best_alignment = Some((pos, req_fuel));
        }
    }

    best_alignment
}

fn main() -> Result<(), anyhow::Error> {
    let crabs: Vec<u16> = std::fs::read_to_string("input.txt")?
        .trim_end()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    println!("part 1: {:?}", find_best_alignment_1(&crabs));
    println!("part 2: {:?}", find_best_alignment_2(&crabs));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_passes_aoc_testcase_1() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(Some((2, 37)), find_best_alignment_1(&crabs));
    }

    #[test]
    fn it_passes_aoc_testcase_2() {
        let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(Some((5, 168)), find_best_alignment_2(&crabs));
    }
}
