use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(PartialEq)]
enum MoveCmd {
    Down(u8),
    Up(u8),
    Forward(u8),
}

#[derive(thiserror::Error, Debug)]
#[error("Unable to parse move command because of unexpected format")]
struct ParseMoveCmdError {}

impl FromStr for MoveCmd {
    type Err = ParseMoveCmdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [command, count] = s.split(" ").collect::<Vec<_>>()[..] {
            let count: u8 = count.parse::<u8>().map_err(|_| ParseMoveCmdError {})?;

            return match command {
                "up" => Ok(MoveCmd::Up(count)),
                "down" => Ok(MoveCmd::Down(count)),
                "forward" => Ok(MoveCmd::Forward(count)),
                _ => Err(ParseMoveCmdError {}),
            };
        }
        Err(ParseMoveCmdError {})
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
    let puzzle_input = File::open("input.txt")?;

    let movements: Vec<MoveCmd> = BufReader::new(puzzle_input)
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?
        .into_iter()
        .map(|m| m.parse())
        .collect::<Result<Vec<MoveCmd>, ParseMoveCmdError>>()?;

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
