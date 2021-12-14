use std::collections::HashMap;

use itermore::IterMore;

fn do_polymerization(
    template: &str,
    step_counter: u8,
    steps: u8,
    rules: &HashMap<(char, char), char>,
) -> String {
    if step_counter == steps {
        return template.to_string();
    }

    let polymerized = template
        .chars()
        .windows()
        .map(|[a, b]| [a, rules[&(a, b)], b].iter().collect::<String>())
        .fold(template.chars().next().unwrap().to_string(), |acc, el| {
            // we crop off the first character in el to make up for windows
            acc + &el.chars().skip(1).collect::<String>()
        });

    do_polymerization(&polymerized, step_counter + 1, steps, rules)
}

/// most common char count minus least common char count
fn most_common_minus_least_common(s: &str) -> Option<usize> {
    let mut letter_counts: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *letter_counts.entry(c).or_insert(0) += 1;
    }
    let mut letter_counts: Vec<(&char, &usize)> = letter_counts.iter().collect::<_>();
    letter_counts.sort_by(|(_, a_count), (_, b_count)| a_count.cmp(b_count));
    Some(letter_counts.last()?.1 - letter_counts.first()?.1)
}

fn parse(input: &str) -> (String, HashMap<(char, char), char>) {
    let mut itr = input.trim_end().lines();
    let template = itr.next().unwrap().to_string();

    let map: HashMap<(char, char), char> = itr
        .filter_map(|line| {
            if let [from, to] = line.split(" -> ").collect::<Vec<_>>()[..] {
                Some((
                    (from.chars().next()?, from.chars().nth(1)?),
                    to.chars().next()?,
                ))
            } else {
                None
            }
        })
        .collect();

    (template, map)
}

fn main() -> Result<(), anyhow::Error> {
    let input = std::fs::read_to_string("input.txt")?;
    let (template, rules) = parse(&input);
    let polymerized = do_polymerization(&template, 0, 10, &rules);
    println!("part 1: {:?}", most_common_minus_least_common(&polymerized));
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn it_passes_aoc_testcase_1() {
        let (template, rules) = parse(&TEST_INPUT);
        let polymerized = do_polymerization(&template, 0, 10, &rules);
        assert_eq!(1588, most_common_minus_least_common(&polymerized).unwrap());
    }

    #[test]
    #[ignore = "too slow"]
    fn it_passes_aoc_testcase_2() {
        let (template, rules) = parse(&TEST_INPUT);
        let polymerized = do_polymerization(&template, 0, 40, &rules);
        assert_eq!(
            2188189693529,
            most_common_minus_least_common(&polymerized).unwrap()
        );
    }
}
