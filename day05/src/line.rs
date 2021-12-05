use std::{
    cmp::{max, min},
    str::FromStr,
};

use ndarray::{Array, ArrayBase, Dim, OwnedRepr};

#[derive(Clone, Debug)]
pub struct Line {
    pub from: (u16, u16),
    pub to: (u16, u16),
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
            // vertical line
            (min(self.from.1, self.to.1)..=max(self.to.1, self.from.1))
                .map(|y| (self.from.0, y))
                .collect()
        } else if self.from.1 == self.to.1 {
            // horizontal line
            (min(self.from.0, self.to.0)..=max(self.to.0, self.from.0))
                .map(|x| (x, self.from.1))
                .collect()
        } else {
            // try to go 45 degrees until we hit the other point
            let mut points = Vec::new();

            let (delta_x, delta_y) = (
                self.to.0 as i16 - self.from.0 as i16,
                self.to.1 as i16 - self.from.1 as i16,
            );

            if delta_x.abs() != delta_y.abs() {
                panic!("Not sure how to get from {:?} to {:?}", self.from, self.to);
            }

            let (step_x, step_y) = (
                if delta_x.is_positive() { 1_i16 } else { -1 },
                if delta_y.is_positive() { 1_i16 } else { -1 },
            );

            let (mut cur_x, mut cur_y) = self.from;

            while points.last() != Some(&self.to) {
                points.push((cur_x, cur_y));

                cur_x = (cur_x as i16 + step_x) as u16;
                cur_y = (cur_y as i16 + step_y) as u16;
            }

            points
        }
    }

    /// Creates a 2d array where points in this line is marked '1'
    /// and other points marked '0'
    pub fn to_2d_array(&self, shape: (usize, usize)) -> ArrayBase<OwnedRepr<u16>, Dim<[usize; 2]>> {
        let mut arr = Array::<u16, _>::zeros(shape);
        for (x, y) in self.points() {
            arr.row_mut(y.into())[usize::from(x)] = 1;
        }
        arr
    }
}
