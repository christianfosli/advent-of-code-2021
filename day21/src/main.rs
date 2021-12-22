fn wrap(n: u8, max: u8) -> u8 {
    ((n - 1) % max) + 1
}

#[derive(Debug, Clone)]
struct Game {
    /// player 1 (current space, score)
    p1: (u8, usize),

    /// player 2 (current space, score)
    p2: (u8, usize),

    dice: u8,
    roll_count: usize,
}

impl Game {
    fn new(p1_start: u8, p2_start: u8) -> Self {
        Game {
            p1: (p1_start, 0),
            p2: (p2_start, 0),
            dice: 0,
            roll_count: 0,
        }
    }

    fn play_round(&mut self) {
        // player 1's turn
        for _ in 0..3 {
            self.dice = wrap(self.dice + 1, 100);
            self.p1.0 = wrap(self.p1.0 + self.dice, 10);
            self.roll_count += 1;
        }
        self.p1.1 += usize::from(self.p1.0);

        if self.loser().is_some() {
            println!("We have a winner! {:?}", self);
            return;
        }

        // player 2's turn
        for _ in 0..3 {
            self.dice = wrap(self.dice + 1, 100);
            self.p2.0 = wrap(self.p2.0 + self.dice, 10);
            self.roll_count += 1;
        }
        self.p2.1 += usize::from(self.p2.0);

        if self.loser().is_some() {
            println!("We have a winner! {:?}", self);
        }
    }

    fn loser(&self) -> Option<usize> {
        if self.p1.1 >= 1000 {
            Some(self.p2.1)
        } else if self.p2.1 >= 1000 {
            Some(self.p1.1)
        } else {
            None
        }
    }
}

fn main() {
    let mut game = Game::new(2, 1);
    while game.loser() == None {
        game.play_round();
    }
    println!("part 1: {}", game.loser().unwrap() * game.roll_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_passes_aoc_testcase() {
        let mut game = Game::new(4, 8);
        while game.loser() == None {
            game.play_round();
        }
        assert_eq!(739785, game.loser().unwrap() * game.roll_count);
    }

    #[test]
    fn it_wraps() {
        assert_eq!(1, wrap(11, 10));
    }
}
