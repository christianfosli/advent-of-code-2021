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

fn main() -> Result<(), anyhow::Error> {
    let signals: Vec<String> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(String::from)
        .collect();

    println!("part 1: {}", count_easy_digits_in_output(&signals));

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
}
