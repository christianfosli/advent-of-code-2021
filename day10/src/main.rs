const OPENING: [char; 4] = ['(', '[', '{', '<'];
const CLOSING: [char; 4] = [')', ']', '}', '>'];

#[derive(thiserror::Error, Debug, PartialEq)]
enum SyntaxError {
    #[error("Unexpected )")]
    Parenthesis,
    #[error("Unexpected ]")]
    Bracket,
    #[error("Unexpected }}")]
    Brace,
    #[error("Unexpected >")]
    Tag,
}

impl SyntaxError {
    fn to_points(&self) -> usize {
        match self {
            SyntaxError::Parenthesis => 3,
            SyntaxError::Bracket => 57,
            SyntaxError::Brace => 1197,
            SyntaxError::Tag => 25137,
        }
    }
}

fn lint(line: &str) -> Result<(), SyntaxError> {
    line.chars().try_fold(Vec::new(), |mut stack, el| {
        if OPENING.contains(&el) {
            stack.push(el);
            Ok(stack)
        } else if CLOSING.contains(&el) {
            let popped = stack.pop();
            match el {
                ')' if popped != Some('(') => Err(SyntaxError::Parenthesis),
                ']' if popped != Some('[') => Err(SyntaxError::Bracket),
                '}' if popped != Some('{') => Err(SyntaxError::Brace),
                '>' if popped != Some('<') => Err(SyntaxError::Tag),
                _ => Ok(stack),
            }
        } else {
            Ok(stack)
        }
    })?;

    Ok(())
}

/// Returns the completion text only. Assumes line is syntactically correct.
fn complete(line: &str) -> String {
    let stack = line.chars().fold(Vec::new(), |mut stack, el| {
        if OPENING.contains(&el) {
            stack.push(el);
        } else if CLOSING.contains(&el) {
            stack.pop();
        }
        stack
    });

    stack
        .iter()
        .rev()
        .map(|c| match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("Unexpected character: {}", c),
        })
        .collect()
}

/// part 1
fn find_syntax_error_score(lines: &[&str]) -> usize {
    lines.iter().fold(0, |acc, el| match lint(el) {
        Ok(_) => acc,
        Err(e) => acc + e.to_points(),
    })
}

/// part 2
fn find_middle_completion_score(lines: &[&str]) -> usize {
    let mut scores: Vec<_> = lines
        .iter()
        .filter(|l| lint(l).is_ok())
        .map(|l| {
            complete(l).chars().fold(0, |points, c| {
                points * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("Unexpected character {}", c),
                    }
            })
        })
        .collect();

    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() -> Result<(), anyhow::Error> {
    let nav_system_code = std::fs::read_to_string("input.txt")?;
    let nav_system_code: Vec<_> = nav_system_code.lines().collect();
    println!("part 1: {}", find_syntax_error_score(&nav_system_code));
    println!("part 2: {}", find_middle_completion_score(&nav_system_code));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAV_SYSTEM_CODE: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn lint_should_error_on_unexpected_brace() {
        assert_eq!(lint("{(}"), Err(SyntaxError::Brace));
    }

    #[test]
    fn it_passes_aoc_testcase_1() {
        assert_eq!(
            26397,
            find_syntax_error_score(&NAV_SYSTEM_CODE.lines().collect::<Vec<_>>())
        );
    }

    #[test]
    fn complete_should_complete() {
        assert_eq!(complete("[({()"), String::from("})]"));
    }

    #[test]
    fn it_passes_aoc_testcase_2() {
        assert_eq!(
            288957,
            find_middle_completion_score(&NAV_SYSTEM_CODE.lines().collect::<Vec<_>>())
        )
    }
}
