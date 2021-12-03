use std::cmp::Ordering;

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

fn main() -> Result<(), anyhow::Error> {
    let diagnostics_report: Vec<String> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(String::from)
        .collect();

    let gamma = find_gamma(&diagnostics_report)?;
    let epsilon = !gamma & 0b0000111111111111;
    println!("part 1: {}", u32::from(gamma) * u32::from(epsilon));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_passes_aoc_testcase_1() -> Result<(), anyhow::Error> {
        let diagnostic_report: Vec<_> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        let gamma = find_gamma(&diagnostic_report)?;
        let epsilon = !gamma & 0b0000000000001111;
        assert_eq!(198, gamma * epsilon);
        Ok(())
    }
}
