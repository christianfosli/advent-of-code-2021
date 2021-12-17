use std::{cmp::Ordering, ops::Range};

const TARGET_AREA: (Range<i32>, Range<i32>) = (207..264, -115..-64);

/// Part 1
/// Finds the initial velocity that causes the proab to reach the highet y position
/// and still eventually be within the target area after any step
fn find_best_velocity(target: &(Range<i32>, Range<i32>)) -> Option<((i32, i32), i32)> {
    let mut best: Option<((i32, i32), i32)> = None;
    for x in 0..100 {
        for y in -100..300 {
            if let Some(hit) = launch((x, y), target) {
                if best.is_none() || hit > best.unwrap().1 {
                    best = Some(((x, y), hit));
                }
            }
        }
    }
    best
}

/// If trajectory hits target area -> returns Some(highest y position)
/// Else -> returns None
fn launch(velocity: (i32, i32), target: &(Range<i32>, Range<i32>)) -> Option<i32> {
    // Ugh, a lot of mutable state here. Couldn't think of a functional solution...
    let (mut x_vel, mut y_vel) = velocity;
    let (mut x, mut y) = (0, 0);
    let mut highest_y = 0;
    let mut hit_target = false;

    while y_vel > 0 || y > target.1.start {
        x += x_vel;
        y += y_vel;

        if y > highest_y {
            highest_y = y;
        }

        if target.0.contains(&x) && target.1.contains(&y) {
            hit_target = true;
        }

        x_vel += match x_vel.cmp(&0) {
            Ordering::Greater => -1,
            Ordering::Equal => 0,
            Ordering::Less => 1,
        };
        y_vel -= 1;
    }

    if hit_target {
        Some(highest_y)
    } else {
        None
    }
}

fn main() {
    println!("part 1: {:?}", find_best_velocity(&TARGET_AREA));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET_AREA: (Range<i32>, Range<i32>) = (20..31, -10..-6);

    #[test]
    fn it_passes_aoc_testcase_1() {
        assert_eq!(Some(((6, 9), 45)), find_best_velocity(&TARGET_AREA));
    }
}
