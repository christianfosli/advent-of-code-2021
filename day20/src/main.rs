use std::{error::Error, iter};

fn enhance(algo: &str, img: &str) -> String {
    let width = img.lines().next().unwrap().len() + 6;
    let height = img.lines().count() + 6;

    // Add some padding
    let img: String = vec!['.'; width]
        .into_iter()
        .chain(iter::once('\n'))
        .chain(vec!['.'; width].into_iter())
        .chain(iter::once('\n'))
        .chain(vec!['.'; width].into_iter())
        .chain(iter::once('\n'))
        .chain(
            img.lines()
                .map(|l| format!("...{}...", l))
                .collect::<Vec<_>>()
                .join("\n")
                .chars(),
        )
        .chain(iter::once('\n'))
        .chain(vec!['.'; width].into_iter())
        .chain(iter::once('\n'))
        .chain(vec!['.'; width].into_iter())
        .chain(iter::once('\n'))
        .chain(vec!['.'; width].into_iter())
        .chain(iter::once('\n'))
        .collect();

    let mut output_img: Vec<char> = Vec::with_capacity(width * height + height);

    // Handle first row
    output_img.extend_from_slice(&vec!['.'; width]);
    output_img.push('\n');

    for y in 1..(height - 1) {
        output_img.push('.'); // first col
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
        output_img.push('.'); // last col
        output_img.push('\n');
    }
    output_img.extend_from_slice(&vec!['.'; width]); // handle last row specially
    output_img.push('\n');

    output_img.iter().collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let algo = input.lines().next().unwrap();
    let img = input.split("\n\n").nth(1).unwrap();
    println!("img:\n{}", &img);
    let enhanced = enhance(algo, img);
    println!("enhanced:\n{}", &enhanced);
    let enhanced = enhance(algo, &enhanced);
    println!("enhanced:\n{}", &enhanced);
    println!("part 1: {}", enhanced.chars().filter(|&c| c == '#').count());

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
        let enhanced = enhance(IMG_ENHANCEMENT_ALGO, PUZZLE);
        println!("enhanced:\n{}", &enhanced);
        let enhanced = enhance(IMG_ENHANCEMENT_ALGO, &enhanced);
        println!("enhanced x2:\n{}", &enhanced);
    }
}
