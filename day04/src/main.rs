use std::str::FromStr;

#[derive(Clone, Debug)]
struct Board {
    /// 5 x 5 board stored as 25 consecutive numbers.
    /// The bool indicates wether the number has been drawn or not.
    /// Get chunks of 5 to get each row
    numbers: [(u8, bool); 25],
}

impl Board {
    fn mark_num(&mut self, n: u8) {
        if let Some(found_pos) = self.numbers.iter().position(|(num, _)| *num == n) {
            self.numbers[found_pos].1 = true;
        }
    }

    fn has_bingo(&self) -> bool {
        if self
            .numbers
            .chunks(5)
            .any(|row| row.iter().all(|(_, marked)| *marked))
        {
            // bingo on a row
            return true;
        }

        let cols = {
            let mut transposed = [(0, false); 25];
            transpose::transpose(&self.numbers, &mut transposed, 5, 5);
            transposed
        };

        if cols
            .chunks(5)
            .any(|row| row.iter().all(|(_, marked)| *marked))
        {
            // bingo on a column
            return true;
        }

        false
    }

    fn score(&self, winning_num: u8) -> usize {
        self.numbers.iter().fold(0, |acc, (num, marked)| {
            if *marked {
                acc
            } else {
                acc + usize::from(*num)
            }
        }) * usize::from(winning_num)
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = [(0, false); 25];
        for (i, line) in s.lines().enumerate() {
            for (j, num) in line.split_whitespace().enumerate() {
                let num: u8 = num.parse()?;
                numbers[i * 5 + j] = (num, false);
            }
        }
        Ok(Board { numbers })
    }
}

/// part 1
fn play_bingo(draws: &[u8], boards: &mut [Board]) -> (u8, Board) {
    for draw in draws {
        for board in &mut *boards {
            board.mark_num(*draw);
            if board.has_bingo() {
                return (*draw, board.clone());
            }
        }
    }
    panic!("nobody got bingo");
}

/// part 2
/// returns the board that got bingo last
fn play_anti_bingo(draws: &[u8], boards: &[Board]) -> (u8, Board) {
    let mut rem_boards = boards.to_vec();

    for draw in draws {
        for board in &mut rem_boards {
            board.mark_num(*draw);
        }

        if rem_boards.len() == 1 && rem_boards[0].has_bingo() {
            return (*draw, rem_boards[0].clone());
        }

        rem_boards.retain(|board| !board.has_bingo());
    }
    panic!("no single board got bingo last")
}

fn main() -> Result<(), anyhow::Error> {
    let input = std::fs::read_to_string("input.txt")?;
    let mut splits = input.split("\n\n");
    let draws: Vec<u8> = splits
        .next()
        .ok_or(anyhow::anyhow!("input.txt missing draw numbers"))?
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let mut boards: Vec<Board> = splits.map(str::parse).collect::<Result<Vec<_>, _>>()?;

    let boards_2 = boards.clone(); // need unmodified boards for part 2

    let (last_drawn, winning_board) = play_bingo(&draws, &mut boards);
    println!(
        "part 1 - winning score: {}",
        winning_board.score(last_drawn)
    );

    let (last_drawn, last_rem_board) = play_anti_bingo(&draws, &boards_2);
    println!("part 2 - last score: {}", last_rem_board.score(last_drawn));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn it_passes_aoc_testcase_1() {
        let mut splits = TEST_INPUT.split("\n\n");
        let draws: Vec<u8> = splits
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let mut boards: Vec<Board> = splits
            .map(|board| board.parse::<Board>().unwrap())
            .collect();

        let (last_drawn, winning_board) = play_bingo(&draws, &mut boards);

        assert_eq!(4512, winning_board.score(last_drawn));
    }

    #[test]
    fn it_passes_aoc_testcase_2() {
        let mut splits = TEST_INPUT.split("\n\n");
        let draws: Vec<u8> = splits
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();

        let boards: Vec<Board> = splits
            .map(|board| board.parse::<Board>().unwrap())
            .collect();

        let (last_drawn, last_rem_board) = play_anti_bingo(&draws, &boards);
        assert_eq!(1924, last_rem_board.score(last_drawn));
    }
}
