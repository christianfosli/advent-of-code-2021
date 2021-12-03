use std::fmt;

#[derive(Copy, Clone, Debug)]
enum MostCommonBit {
    High(u16),
    Low(u16),
    Equally,
}

impl fmt::Display for MostCommonBit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MostCommonBit::High(_) => write!(f, "1"),
            MostCommonBit::Low(_) => write!(f, "0"),
            MostCommonBit::Equally => write!(f, ""),
        }
    }
}

fn find_gamma_and_epsilon(report: &[String], test: bool) -> Result<(u16, u16), anyhow::Error> {
    let most_common_bits = report
        .into_iter()
        .fold([MostCommonBit::Equally; 12], |mut acc, x| {
            for (pos, bit) in x.chars().enumerate() {
                let new_x = match bit {
                    '0' => match acc[pos] {
                        MostCommonBit::High(n) => {
                            if n == 1 {
                                MostCommonBit::Equally
                            } else {
                                MostCommonBit::High(n - 1)
                            }
                        }
                        MostCommonBit::Low(n) => MostCommonBit::Low(n + 1),
                        MostCommonBit::Equally => MostCommonBit::Low(1),
                    },
                    '1' => match acc[pos] {
                        MostCommonBit::High(n) => MostCommonBit::High(n + 1),
                        MostCommonBit::Low(n) => {
                            if n == 1 {
                                MostCommonBit::Equally
                            } else {
                                MostCommonBit::Low(n - 1)
                            }
                        }
                        MostCommonBit::Equally => MostCommonBit::High(1),
                    },
                    _ => unreachable!(""),
                };

                acc[pos] = new_x;
            }

            acc
        });

    let gamma = most_common_bits
        .iter()
        .fold("".to_owned(), |acc, x| format!("{}{}", acc, x));

    let gamma = u16::from_str_radix(&gamma, 2)?;

    let epsilon = if test {
        !gamma & 0b0000000000001111
    } else {
        !gamma & 0b0000111111111111
    };

    Ok((gamma, epsilon))
}

fn main() -> Result<(), anyhow::Error> {
    let report: Vec<String> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(String::from)
        .collect();

    if let Ok((gamma, epsilon)) = find_gamma_and_epsilon(&report, false) {
        println!("{}", u64::from(gamma) * u64::from(epsilon));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_passes_aoc_testcase() {
        let diagnostic_report: Vec<_> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .into_iter()
        .map(String::from)
        .collect();

        if let Ok((gamma, epsilon)) = find_gamma_and_epsilon(&diagnostic_report, true) {
            assert_eq!(198, u64::from(gamma) * u64::from(epsilon));
        } else {
            panic!("find_gamma_and_epsilon returned an error :-(");
        }
    }
}
