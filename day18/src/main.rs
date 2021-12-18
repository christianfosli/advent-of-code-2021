use std::{error::Error, ops};

#[derive(Debug, PartialEq, Clone)]
struct SnailfishNum {
    left: Node,
    right: Node,
}

#[derive(Debug, PartialEq, Clone)]
enum Node {
    Leaf(usize),
    Pair(Box<SnailfishNum>),
}

impl Node {
    fn reduce(&mut self, num_parents: usize, on_explode: fn(usize, usize)) {
        match self {
            Self::Pair(p) if num_parents == 4 => {
                // Explode
                on_explode(p.left.unwrap(), p.right.unwrap());
                *self = Node::Leaf(0);
            }
            Self::Leaf(ref n) if *n >= 10 => {
                // Split
                *self = self.split();
            }
            Self::Pair(p) => {
                // Keep going
                p.reduce(num_parents);
            }
            Self::Leaf(_) => {} // No more action applies
        }
    }

    fn split(&self) -> Self {
        if let Node::Leaf(n) = self {
            Node::Pair(Box::new(SnailfishNum {
                left: Node::Leaf(n / 2),            // div 2, round down
                right: Node::Leaf((n + 2 - 1) / 2), // div 2, round up
            }))
        } else {
            panic!("Tried to split a pair");
        }
    }

    fn unwrap(&self) -> usize {
        match self {
            Self::Leaf(n) => *n,
            _ => panic!("Tried to unwrap a pair"),
        }
    }
}

impl SnailfishNum {
    fn reduce(&mut self, num_parents: usize) {
        self.right.reduce(num_parents + 1, |l, r| {
            println!(
                "My right child exploded! I need to add {:?} to some leafs",
                (l, r)
            );
        });
        self.left.reduce(num_parents + 1, |l, r| {
            println!(
                "My left child exploded! I need to add {:?} to some leafs",
                (l, r)
            );
        });
    }
}

impl ops::Add<SnailfishNum> for SnailfishNum {
    type Output = SnailfishNum;

    fn add(self, rhs: SnailfishNum) -> Self::Output {
        let left = Node::Pair(Box::new(self));
        let right = Node::Pair(Box::new(rhs));

        let mut added = SnailfishNum { left, right };

        added.reduce(0);

        added
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_leafs() {
        let ones = SnailfishNum {
            left: Node::Leaf(1),
            right: Node::Leaf(1),
        };

        let twoes = SnailfishNum {
            left: Node::Leaf(2),
            right: Node::Leaf(2),
        };

        assert_eq!(
            ones + twoes,
            SnailfishNum {
                left: Node::Pair(Box::new(SnailfishNum {
                    left: Node::Leaf(1),
                    right: Node::Leaf(1),
                })),
                right: Node::Pair(Box::new(SnailfishNum {
                    left: Node::Leaf(2),
                    right: Node::Leaf(2),
                })),
            }
        );
    }

    #[test]
    fn it_adds_leafs_more_complex() {
        let nums = vec![
            SnailfishNum {
                left: Node::Leaf(1),
                right: Node::Leaf(1),
            },
            SnailfishNum {
                left: Node::Leaf(2),
                right: Node::Leaf(2),
            },
            SnailfishNum {
                left: Node::Leaf(3),
                right: Node::Leaf(3),
            },
            SnailfishNum {
                left: Node::Leaf(4),
                right: Node::Leaf(4),
            },
            SnailfishNum {
                left: Node::Leaf(5),
                right: Node::Leaf(5),
            },
        ];

        let sum = nums.into_iter().reduce(|acc, el| acc + el).unwrap();

        assert_eq!(
            sum,
            SnailfishNum {
                left: Node::Pair(Box::new(SnailfishNum {
                    left: Node::Pair(Box::new(SnailfishNum {
                        left: Node::Pair(Box::new(SnailfishNum {
                            left: Node::Leaf(5),
                            right: Node::Leaf(0),
                        })),
                        right: Node::Pair(Box::new(SnailfishNum {
                            left: Node::Leaf(7),
                            right: Node::Leaf(4),
                        })),
                    })),
                    right: Node::Pair(Box::new(SnailfishNum {
                        left: Node::Leaf(5),
                        right: Node::Leaf(5),
                    })),
                })),
                right: Node::Pair(Box::new(SnailfishNum {
                    left: Node::Leaf(6),
                    right: Node::Leaf(6),
                })),
            }
        )
    }
}
