use std::collections::{HashMap, HashSet};

/// part 1 - count 1, 4, 7 or 8 in output signal
fn count_easy_digits_in_output(signals: &[String]) -> usize {
    let is_1 = |digit: &str| digit.len() == 2;
    let is_4 = |digit: &str| digit.len() == 4;
    let is_7 = |digit: &str| digit.len() == 3;
    let is_8 = |digit: &str| digit.len() == 7;
    signals
        .iter()
        .map(|s| s.split(" | ").nth(1).unwrap())
        .fold(0, |acc, out_sig| {
            acc + out_sig
                .split(' ')
                .filter(|digit| is_1(digit) || is_4(digit) || is_7(digit) || is_8(digit))
                .count()
        })
}

/// part 2 -- DOES NOT WORK YET!
fn add_output_values(signals: &[String]) -> usize {
    signals.iter().map(|s| find_output(&s)).sum()
}

fn find_output(signals: &String) -> usize {
    if let [signals, output] = signals.split(" | ").collect::<Vec<_>>()[..] {
        let mut translations: HashMap<char, char> = HashMap::new();

        let c_or_f: HashSet<char> = signals
            .split(' ')
            .find(|d| d.len() == 2)
            .unwrap()
            .chars()
            .collect();

        let f = c_or_f
            .iter()
            .find(|&&x| {
                signals
                    .split(' ')
                    .all(|digit| digit.chars().any(|c| c == x))
            })
            .unwrap();

        translations.insert('f', *f);

        translations.insert(
            'c',
            *(&c_or_f - &HashSet::from([*f])).iter().next().unwrap(),
        );

        let a = signals
            .split(' ')
            .find(|d| d.len() == 3) // 4: a or c or f
            .unwrap()
            .chars()
            .find(|&x| x != *f && x != translations[&'c'])
            .unwrap();

        translations.insert('a', a);

        let _b_c_d_or_f: HashSet<char> = signals
            .split(' ')
            .find(|d| d.len() == 4)
            .unwrap()
            .chars()
            .collect();

        // TODO: Keep working out what letter map to what
        return 0;
    }
    panic!("invalid format");
}

fn main() -> Result<(), anyhow::Error> {
    let signals: Vec<String> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(String::from)
        .collect();

    println!("part 1: {}", count_easy_digits_in_output(&signals));

    println!(
        "{:?}",
        signals.iter().all(|s| s.split(' ').any(|d| d.len() == 3))
    );

    println!("part 2: {}", add_output_values(&signals));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_SIGNALS: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn it_passes_aoc_testcase_1() {
        let signals: Vec<String> = EXAMPLE_SIGNALS.lines().map(String::from).collect();
        assert_eq!(26, count_easy_digits_in_output(&signals));
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn it_passes_aoc_testcase_2() {
        let signals: Vec<String> = EXAMPLE_SIGNALS.lines().map(String::from).collect();
        assert_eq!(61_229, add_output_values(&signals));
    }
}
