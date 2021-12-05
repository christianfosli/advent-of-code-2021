use std::{
    cmp::{max, min},
    str::FromStr,
};

use ndarray::prelude::*;

#[derive(Clone, Debug)]
struct Line {
    from: (u16, u16),
    to: (u16, u16),
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // s: x1,y1 -> x2, y2
        if let [from, to] = s.split(" -> ").collect::<Vec<_>>()[..] {
            if let [x1, y1] = from
                .split(',')
                .map(str::parse)
                .collect::<Result<Vec<_>, _>>()?[..]
            {
                if let [x2, y2] = to
                    .split(',')
                    .map(str::parse)
                    .collect::<Result<Vec<_>, _>>()?[..]
                {
                    return Ok(Line {
                        from: (x1, y1),
                        to: (x2, y2),
                    });
                }
            }
        }

        Err(anyhow::anyhow!(
            "Parse error: Invalid format. Expected 'x1,y1 -> x2,y2'. Got {}",
            s
        ))
    }
}

impl Line {
    fn points(&self) -> Vec<(u16, u16)> {
        if self.from.0 == self.to.0 {
            return (min(self.from.1, self.to.1)..=max(self.to.1, self.from.1))
                .map(|y| (self.from.0, y))
                .collect();
        } else if self.from.1 == self.to.1 {
            return (min(self.from.0, self.to.0)..=max(self.to.0, self.from.0))
                .map(|x| (x, self.from.1))
                .collect();
        }
        unimplemented!("Not impl for lines that are not horizontal/vertical only")
    }

    /// Creates a 2d array where points in this line is marked '1'
    /// and other points marked '0'
    fn to_2d_array(
        &self,
        shape: (usize, usize),
    ) -> ArrayBase<ndarray::OwnedRepr<u16>, Dim<[usize; 2]>> {
        let mut arr = Array::<u16, _>::zeros(shape);
        for (x, y) in self.points() {
            arr.row_mut(y.into())[usize::from(x)] = 1;
        }
        arr
    }
}

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
        "part 1 - num intersections: {}",
        count_intersections(&horiz_or_vert, (1000, 1000))
    );

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
}
