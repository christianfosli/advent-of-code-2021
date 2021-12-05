mod line;
use crate::line::Line;

/// Counts intersecting points by converting each line to a matrix
/// with the line's points as 1's and rest 0's,
/// adding the matrices together and checking how many points intersected.
///
/// A bit overkill way to find intersections, but kinda fun :-)
fn count_intersections(lines: &[Line], shape: (usize, usize)) -> usize {
    let matrix = lines
        .iter()
        .map(|l| l.to_2d_array(shape))
        .reduce(|acc, el| acc + el)
        .unwrap();

    //dbg!(&matrix);

    matrix.iter().filter(|x| **x > 1).count()
}

fn main() -> Result<(), anyhow::Error> {
    let lines: Vec<Line> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let horiz_or_vert: Vec<_> = lines
        .iter()
        .filter(|l| l.from.0 == l.to.0 || l.from.1 == l.to.1)
        .cloned()
        .collect();

    println!(
        "part 1: {}",
        count_intersections(&horiz_or_vert, (1000, 1000))
    );

    println!("part 2: {}", count_intersections(&lines, (1000, 1000)));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> Vec<Line> {
        vec![
            Line {
                from: (0, 9),
                to: (5, 9),
            },
            Line {
                from: (8, 0),
                to: (0, 8),
            },
            Line {
                from: (9, 4),
                to: (3, 4),
            },
            Line {
                from: (2, 2),
                to: (2, 1),
            },
            Line {
                from: (7, 0),
                to: (7, 4),
            },
            Line {
                from: (6, 4),
                to: (2, 0),
            },
            Line {
                from: (0, 9),
                to: (2, 9),
            },
            Line {
                from: (3, 4),
                to: (1, 4),
            },
            Line {
                from: (0, 0),
                to: (8, 8),
            },
            Line {
                from: (5, 5),
                to: (8, 2),
            },
        ]
    }

    #[test]
    fn it_passes_aoc_testcase_1() {
        let lines: Vec<_> = get_test_input()
            .iter()
            .filter(|l| l.from.0 == l.to.0 || l.from.1 == l.to.1)
            .cloned()
            .collect();

        assert_eq!(5, count_intersections(&lines, (10, 10)));
    }

    #[test]
    fn it_passes_aoc_testcase_2() {
        let lines = get_test_input();
        assert_eq!(12, count_intersections(&lines, (10, 10)));
    }

    #[test]
    fn it_fails() {
        panic!("noooo");
    }
}
