use std::error::Error;

fn move_along(seafloor: &[char], width: usize) -> Vec<char> {
    let mut moved: Vec<char> = seafloor.iter().copied().collect();

    // first the east-facing cucumbers move
    for (i, &c) in seafloor.iter().enumerate().filter(|(_, &c)| c == '>') {
        let move_to = if (i + 1) % width == 0 {
            // right most col moving to left-most col
            i + 1 - width
        } else {
            i + 1
        };

        if seafloor[move_to] == '.' {
            moved[move_to] = c;
            moved[i] = '.';
        }
    }

    // then the south facing cucumbers move
    let seafloor = moved.to_vec();
    for (i, &c) in seafloor.iter().enumerate().filter(|(_, &c)| c == 'v') {
        let move_to = if i >= (seafloor.len() - width) {
            // bottom row moving to top row
            i % width
        } else {
            i + width
        };
        if seafloor[move_to] == '.' {
            moved[move_to] = c;
            moved[i] = '.';
        }
    }

    moved
}

fn main() -> Result<(), Box<dyn Error>> {
    let seafloor = std::fs::read_to_string("input.txt")?;
    let width = seafloor.lines().next().unwrap().len();
    let mut seafloor = seafloor.replace('\n', "").chars().collect::<Vec<_>>();
    for step in 1..usize::MAX {
        let new_seafloor = move_along(&seafloor, width);
        if new_seafloor == seafloor {
            println!("YES! {}", step);
            break;
        }
        seafloor = new_seafloor;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_moves_as_expected() {
        let seafloor = "...>...
.......
......>
v.....>
......>
.......
..vvv..";

        let moved = (0..4).fold(
            seafloor.replace('\n', "").chars().collect::<Vec<_>>(),
            |seafloor, _| move_along(&seafloor, 7),
        );

        assert_eq!(
            moved,
            ">......
..v....
..>.v..
.>.v...
...>...
.......
v......"
                .replace('\n', "")
                .chars()
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn it_changes_every_time_until_step_58() {
        let seafloor = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        let width = seafloor.lines().next().unwrap().len();
        let mut seafloor = seafloor.replace('\n', "").chars().collect::<Vec<_>>();
        for i in 1..=58 {
            let new_seafloor = move_along(&seafloor, width);
            println!("step {}", i);
            if i == 58 {
                assert_eq!(seafloor, new_seafloor);
            } else {
                assert_ne!(seafloor, new_seafloor);
            }
            seafloor = new_seafloor;
        }
    }
}
