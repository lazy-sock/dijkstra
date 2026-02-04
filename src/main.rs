use crate::graph::Graph;
use crate::graph::get_random_graph;

fn main() {
    let graph = get_random_graph();
    println!("{}", graph);
    println!("{:?}", dijkstra(graph, 0));
}

fn dijkstra(graph: Graph, source: i32) -> Vec<i32> {
    let mut dist = vec![]; // current discovered cost of getting to every node
    let mut unvisited: Vec<i32> = vec![];
    for i in 0..graph.matrix.len() {
        dist.push(100000);
        unvisited.push(i as i32);
    }
    dist[source as usize] = 0;
    while !unvisited.is_empty() {
        unvisited.sort_by_key(|&node| dist[node as usize]);
        let u = unvisited.pop();
        for (index, edge) in graph.matrix[u.unwrap() as usize].iter().enumerate() {
            if *edge == 0 {
                continue;
            }
            let alt = edge + dist[u.unwrap() as usize];
            if alt < dist[index] {
                dist[index] = alt;
            }
        }
    }

    dist
}

mod graph;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let matrix = vec![vec![0, 2], vec![2, 0]];
        assert_eq!(dijkstra(Graph { matrix }, 0), [0, 2]);
    }
}
