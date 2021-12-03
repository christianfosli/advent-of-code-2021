use std::cmp::Ordering;

/// part 1
fn find_gamma(report: &[String]) -> Result<u16, anyhow::Error> {
    let mut counts: Vec<isize> = vec![0; report[0].len()];

    for number in report {
        for (pos, bit) in number.chars().enumerate() {
            match bit {
                '0' => counts[pos] -= 1,
                '1' => counts[pos] += 1,
                _ => anyhow::bail!("invalid format - non binary"),
            };
        }
    }

    let gamma: String = counts
        .iter()
        .map(|c| match c.cmp(&0) {
            Ordering::Less => '0',    // majority of bits are low
            Ordering::Greater => '1', // majority of bits are high
            Ordering::Equal => panic!("not sure how to handle when no bit 'wins'"),
        })
        .collect();

    Ok(u16::from_str_radix(&gamma, 2)?)
}

/// part 2
fn find_oxygen_generator_rating(report: &[String], offset: usize) -> Result<u16, anyhow::Error> {
    if report.len() == 1 {
        return Ok(u16::from_str_radix(&report[0], 2)?);
    }

    let high_or_low = {
        let considered = report.iter().filter_map(|num| num.chars().nth(offset));
        let highs = considered.filter(|bit| *bit == '1').count();
        if highs >= report.len() - highs {
            '1'
        } else {
            '0'
        }
    };

    let rem_numbers: Vec<String> = report
        .iter()
        .filter(|num| num.chars().nth(offset) == Some(high_or_low))
        .cloned()
        .collect();

    find_oxygen_generator_rating(&rem_numbers, offset + 1)
}

/// part 2
fn find_co2_scrubber_rating(report: &[String], offset: usize) -> Result<u16, anyhow::Error> {
    if report.len() == 1 {
        return Ok(u16::from_str_radix(&report[0], 2)?);
    }

    let high_or_low = {
        let considered = report.iter().filter_map(|num| num.chars().nth(offset));
        let lows = considered.filter(|bit| *bit == '0').count();
        if lows <= report.len() - lows {
            '0'
        } else {
            '1'
        }
    };

    let rem_numbers: Vec<String> = report
        .iter()
        .filter(|num| num.chars().nth(offset) == Some(high_or_low))
        .cloned()
        .collect();

    find_co2_scrubber_rating(&rem_numbers, offset + 1)
}

fn main() -> Result<(), anyhow::Error> {
    let diagnostics_report: Vec<String> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(String::from)
        .collect();

    let gamma = find_gamma(&diagnostics_report)?;
    let epsilon = !gamma & 0b0000_1111_1111_1111;
    println!("part 1: {}", u32::from(gamma) * u32::from(epsilon));

    let oxygen_rating = find_oxygen_generator_rating(&diagnostics_report, 0)?;
    let co2_rating = find_co2_scrubber_rating(&diagnostics_report, 0)?;
    println!(
        "part 2: {}",
        u32::from(oxygen_rating) * u32::from(co2_rating)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_report() -> Vec<String> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }

    #[test]
    fn it_passes_aoc_testcase_1() -> Result<(), anyhow::Error> {
        let gamma = find_gamma(&test_report())?;
        let epsilon = !gamma & 0b1111;
        assert_eq!(198, gamma * epsilon);
        Ok(())
    }

    #[test]
    fn it_passes_aoc_testcase_2() -> Result<(), anyhow::Error> {
        let report = test_report();
        let oxygen_rating = find_oxygen_generator_rating(&report, 0)?;
        let co2_rating = find_co2_scrubber_rating(&report, 0)?;
        assert_eq!(230, oxygen_rating * co2_rating);
        Ok(())
    }
}
