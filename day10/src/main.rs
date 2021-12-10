#[derive(thiserror::Error, Debug)]
enum SyntaxError {
    #[error("Unexpected )")]
    Paranthesis,
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
            SyntaxError::Paranthesis => 3,
            SyntaxError::Bracket => 57,
            SyntaxError::Brace => 1197,
            SyntaxError::Tag => 25137,
        }
    }
}

fn lint(line: &str) -> Result<(), SyntaxError> {
    const OPENING: [char; 4] = ['(', '[', '{', '<'];
    const CLOSING: [char; 4] = [')', ']', '}', '>'];

    line.chars().try_fold(Vec::new(), |mut stack, el| {
        if OPENING.contains(&el) {
            stack.push(el);
            Ok(stack)
        } else if CLOSING.contains(&el) {
            let popped = stack.pop();
            match el {
                ')' if popped != Some('(') => Err(SyntaxError::Paranthesis),
                ']' if popped != Some('[') => Err(SyntaxError::Bracket),
                '}' if popped != Some('{') => Err(SyntaxError::Brace),
                '>' if popped != Some('<') => Err(SyntaxError::Tag),
                _ => Ok(stack),
            }
        } else {
            panic!("Unexpected character: {}", el);
        }
    })?;

    Ok(())
}

/// part 1
fn find_syntax_error_score(lines: &[&str]) -> usize {
    lines.iter().fold(0, |acc, el| match lint(el) {
        Ok(_) => acc,
        Err(e) => acc + e.to_points(),
    })
}

fn main() -> Result<(), anyhow::Error> {
    let nav_system_code = std::fs::read_to_string("input.txt")?;
    let nav_system_code: Vec<_> = nav_system_code.lines().collect();
    println!("part 1: {}", find_syntax_error_score(&nav_system_code));
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
    fn it_passes_aoc_testcase_1() {
        assert_eq!(
            26397,
            find_syntax_error_score(&NAV_SYSTEM_CODE.lines().collect::<Vec<_>>())
        );
    }
}
