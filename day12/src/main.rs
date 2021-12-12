use petgraph::dot::Dot;
use petgraph::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Cave {
    Start,
    Big(char),
    Small(char),
    End,
}

fn parse(map: &str) -> GraphMap<Cave, (), Undirected> {
    let map: Vec<_> = map
        .lines()
        .map(|line| {
            if let [from, two] = line
                .split('-')
                .map(|c| match c {
                    "start" => Cave::Start,
                    "end" => Cave::End,
                    _ => {
                        let ch = c.chars().next().unwrap();
                        if ch.is_lowercase() {
                            Cave::Small(ch)
                        } else {
                            Cave::Big(ch)
                        }
                    }
                })
                .collect::<Vec<_>>()[..]
            {
                return (from, two);
            }
            panic!("Invalid format");
        })
        .collect();

    UnGraphMap::<_, ()>::from_edges(&map)
}

/// TODO: This doesn't work )-:
fn count_paths(
    graph: &GraphMap<Cave, (), Undirected>,
    from: Cave,
    to: Cave,
    visisted_bigs: &mut Vec<Cave>,
) -> usize {
    if from == to {
        return 1;
    }

    if let Cave::Big(_) = from {
        visisted_bigs.push(from);
    }

    let mut count = 0;

    for neigh in graph.neighbors(from) {
        if neigh == Cave::Start || visisted_bigs.contains(&neigh) {
            continue;
        }
        count += count_paths(graph, neigh, to, visisted_bigs);
    }

    count
}

fn main() -> Result<(), anyhow::Error> {
    let map = std::fs::read_to_string("input.txt")?;
    let g = parse(&map);
    println!("{:?}", Dot::new(&g));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MAP: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    #[test]
    fn it_passes_aoc_testcase_1() {
        let g = parse(TEST_MAP);
        println!("{:?}", Dot::new(&g));

        assert_eq!(count_paths(&g, Cave::Start, Cave::End, &mut Vec::new()), 10);
    }
}
