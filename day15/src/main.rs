use anyhow::anyhow;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

/// Parses string s into graph of <risklevel, weight=risklevel-into, Directed>
fn parse(s: &str) -> Result<Graph<u32, u32, Directed>, anyhow::Error> {
    let rowlen = s
        .lines()
        .next()
        .ok_or(anyhow!("unexpected empty string"))?
        .len();

    let mut graph: Graph<u32, u32, Directed> = Graph::new();

    let levels: Vec<u32> = s
        .replace('\n', "")
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| anyhow!("expected digit but got {}", c))
        })
        .collect::<Result<Vec<_>, _>>()?;

    for l in &levels {
        graph.add_node(*l);
    }

    for i in 0..levels.len() {
        // Add edge to left unless were at the leftmost column
        if i % rowlen != 0 {
            graph.add_edge(NodeIndex::new(i), NodeIndex::new(i - 1), levels[i - 1]);
        }

        // Add edge to right unless we're at the rightmost column
        if i % rowlen != rowlen - 1 {
            graph.add_edge(NodeIndex::new(i), NodeIndex::new(i + 1), levels[i + 1]);
        }

        // Add edge above unless were at the top row
        if i >= rowlen {
            graph.add_edge(
                NodeIndex::new(i),
                NodeIndex::new(i - rowlen),
                levels[i - rowlen],
            );
        }

        // Add edge below unless were at the bottom row
        if i < levels.len() - rowlen {
            graph.add_edge(
                NodeIndex::new(i),
                NodeIndex::new(i + rowlen),
                levels[i + rowlen],
            );
        }
    }

    Ok(graph)
}

fn main() -> Result<(), anyhow::Error> {
    let riskmap = std::fs::read_to_string("input.txt")?;
    let graph = parse(&riskmap)?;
    let goal = NodeIndex::new(graph.node_count() - 1);
    let res = dijkstra(&graph, NodeIndex::new(0), Some(goal), |edge| *edge.weight());
    println!("part 1: {}", res[&goal]);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const RISK_LEVEL_MAP: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn it_passes_aoc_testcase_1() {
        let graph = parse(RISK_LEVEL_MAP).unwrap();
        let goal = NodeIndex::new(99);
        let res = dijkstra(&graph, NodeIndex::new(0), Some(goal), |edge| *edge.weight());

        //println!("{:?}", Dot::new(&graph));
        assert_eq!(40, res[&goal]);
    }
}
