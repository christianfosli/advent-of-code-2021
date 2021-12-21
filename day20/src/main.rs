use std::{error::Error, iter};

fn enhance(algo: &str, img: &str, outside: char) -> (String, char) {
    let width = img.lines().next().unwrap().len() + 6;
    let height = img.lines().count() + 6;

    // Add some padding
    let img: String = vec![outside; width]
        .into_iter()
        .chain(iter::once('\n'))
        .chain(vec![outside; width].into_iter())
        .chain(iter::once('\n'))
        .chain(vec![outside; width].into_iter())
        .chain(iter::once('\n'))
        .chain(
            img.lines()
                .map(|l| {
                    format!(
                        "{}{}{}{}{}{}{}",
                        outside, outside, outside, l, outside, outside, outside
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
                .chars(),
        )
        .chain(iter::once('\n'))
        .chain(vec![outside; width].into_iter())
        .chain(iter::once('\n'))
        .chain(vec![outside; width].into_iter())
        .chain(iter::once('\n'))
        .chain(vec![outside; width].into_iter())
        .chain(iter::once('\n'))
        .collect();

    let mut output_img: Vec<char> = Vec::with_capacity(width * height);

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let index = img
                .lines()
                .skip(y - 1)
                .take(3)
                .map(|row| {
                    row.chars()
                        .skip(x - 1)
                        .take(3)
                        .fold(String::from(""), |acc, el| match el {
                            '#' => acc + "1",
                            '.' => acc + "0",
                            _ => panic!("Invalid character"),
                        })
                })
                .reduce(|acc, el| acc + &el)
                .map(|bitstr| u16::from_str_radix(&bitstr, 2).unwrap())
                .unwrap();

            output_img.push(algo.chars().nth(index.into()).unwrap());
        }
        output_img.push('\n');
    }

    let new_outside = algo
        .chars()
        .nth(if outside == '.' { 0 } else { 0b1_1111_1111 })
        .unwrap();

    (output_img.iter().collect(), new_outside)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let algo = input.lines().next().unwrap();
    let img = input.split("\n\n").nth(1).unwrap();
    println!("img:\n{}", &img);
    let (enhanced, outside) = enhance(algo, img, '.');
    println!("enhanced:\n{}", &enhanced);
    let (enhanced, _) = enhance(algo, &enhanced, outside);
    println!("enhanced x2:\n{}", &enhanced);
    println!("part 1: {}", enhanced.chars().filter(|&c| c == '#').count());

    // My solution is a bit slow for part 2 -- Run with `--release` and wait a few seconds :-)
    let (p2, _) = (0..50).fold((img.to_owned(), '.'), |(img, out), _| {
        enhance(algo, &img, out)
    });
    println!("part 2: {}", p2.chars().filter(|&c| c == '#').count());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const IMG_ENHANCEMENT_ALGO: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";

    const PUZZLE: &str = "#..#.
#....
##..#
..#..
..###";

    #[test]
    fn it_enhances_image() {
        println!("initial:\n{}", PUZZLE);
        let (enhanced, outside) = enhance(IMG_ENHANCEMENT_ALGO, PUZZLE, '.');
        println!("enhanced:\n{}", &enhanced);
        let (enhanced, _) = enhance(IMG_ENHANCEMENT_ALGO, &enhanced, outside);
        println!("enhanced x2:\n{}", &enhanced);
    }
}
