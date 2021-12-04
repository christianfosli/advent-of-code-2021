use std::{fs, str::FromStr};

#[derive(PartialEq)]
enum MoveCmd {
    Down(u8),
    Up(u8),
    Forward(u8),
}

impl FromStr for MoveCmd {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [command, count] = s.split(' ').collect::<Vec<_>>()[..] {
            let count: u8 = count.parse()?;

            return match command {
                "up" => Ok(MoveCmd::Up(count)),
                "down" => Ok(MoveCmd::Down(count)),
                "forward" => Ok(MoveCmd::Forward(count)),
                _ => Err(anyhow::anyhow!("Parse error: Invalid command")),
            };
        }
        Err(anyhow::anyhow!("Parse error: Invalid format"))
    }
}

struct SubmarinePos {
    /// Horizontal position
    hpos: u32,
    depth: u32,
    aim: u32,
}

impl SubmarinePos {
    fn from(movements: &[MoveCmd]) -> Self {
        movements.iter().fold(
            SubmarinePos {
                hpos: 0,
                depth: 0,
                aim: 0,
            },
            |acc, x| match x {
                MoveCmd::Down(n) => SubmarinePos {
                    aim: acc.aim + u32::from(*n),
                    ..acc
                },
                MoveCmd::Up(n) => SubmarinePos {
                    aim: acc.aim - u32::from(*n),
                    ..acc
                },
                MoveCmd::Forward(n) => SubmarinePos {
                    hpos: acc.hpos + u32::from(*n),
                    depth: acc.depth + acc.aim * u32::from(*n),
                    ..acc
                },
            },
        )
    }
}

fn main() -> Result<(), anyhow::Error> {
    let movements: Vec<MoveCmd> = fs::read_to_string("input.txt")?
        .lines()
        .into_iter()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let SubmarinePos { hpos, depth, .. } = SubmarinePos::from(&movements);
    println!("{}", hpos * depth);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_passes_aoc_testcase_part_2() {
        let planned_course = vec![
            MoveCmd::Forward(5),
            MoveCmd::Down(5),
            MoveCmd::Forward(8),
            MoveCmd::Up(3),
            MoveCmd::Down(8),
            MoveCmd::Forward(2),
        ];

        let pos = SubmarinePos::from(&planned_course);
        assert_eq!(900, pos.hpos * pos.depth);
    }
}
