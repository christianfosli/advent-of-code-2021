use ndarray::{prelude::*, OwnedRepr, ShapeError};

fn to_matrix(heatmap: &str) -> Result<ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>>, ShapeError> {
    let height = heatmap.lines().count();
    let width = heatmap.lines().next().unwrap().len();

    Array::from_shape_vec(
        (height, width),
        heatmap
            .replace('\n', "")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>(),
    )
}

/// part 1
fn sum_risk_level_of_lows(heatmap: &ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>>) -> usize {
    if let [height, width] = heatmap.shape()[..] {
        let mut sum = 0;

        for row in 0..height {
            for col in 0..width {
                let n = heatmap[[row, col]];

                if (row == 0 || n < heatmap[[row - 1, col]])
                    && (row == height - 1 || n < heatmap[[row + 1, col]])
                    && (col == 0 || n < heatmap[[row, col - 1]])
                    && (col == width - 1 || n < heatmap[[row, col + 1]])
                {
                    sum += usize::from(n + 1);
                }
            }
        }

        return sum;
    }
    unreachable!("the heatmap is a 2d array so shape always has a width and height");
}

/// part 2
fn _multiply_sizeof_three_largest_basins(
    _heatmap: &ArrayBase<OwnedRepr<u8>, Dim<[usize; 2]>>,
) -> usize {
    todo!()
}

fn main() -> Result<(), anyhow::Error> {
    let heatmap = std::fs::read_to_string("input.txt")?;
    let heatmap = to_matrix(&heatmap)?;
    println!("heatmap:\n{}\n", &heatmap);

    println!("part 1: {}", sum_risk_level_of_lows(&heatmap));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_HEATMAP: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn it_passes_aoc_testcase_1() {
        let heatmap = to_matrix(TEST_HEATMAP).unwrap();
        assert_eq!(15, sum_risk_level_of_lows(&heatmap).unwrap())
    }
}
