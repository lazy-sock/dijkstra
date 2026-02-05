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
        dist.push(i32::MAX);
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
    fn test_dijkstra_single_node() {
        let matrix = vec![vec![0]];
        assert_eq!(dijkstra(Graph { matrix }, 0), [0]);
    }

    #[test]
    fn test_dijkstra_two_nodes() {
        let matrix = vec![vec![0, 2], vec![2, 0]];
        assert_eq!(dijkstra(Graph { matrix }, 0), [0, 2]);
    }

    #[test]
    fn test_dijkstra_three_nodes_linear() {
        let matrix = vec![vec![0, 1, i32::MAX], vec![1, 0, 3], vec![i32::MAX, 3, 0]];
        assert_eq!(dijkstra(Graph { matrix }, 0), [0, 1, 4]);
    }

    #[test]
    fn test_dijkstra_three_nodes_triangle() {
        let matrix = vec![vec![0, 2, 5], vec![2, 0, 1], vec![5, 1, 0]];
        assert_eq!(dijkstra(Graph { matrix }, 0), [0, 2, 3]);
    }

    #[test]
    fn test_dijkstra_four_nodes() {
        let matrix = vec![
            vec![0, 4, 2, i32::MAX],
            vec![4, 0, 1, 5],
            vec![2, 1, 0, 8],
            vec![i32::MAX, 5, 8, 0],
        ];
        assert_eq!(dijkstra(Graph { matrix }, 0), [0, 3, 2, 8]);
    }

    #[test]
    fn test_dijkstra_from_different_source() {
        let matrix = vec![vec![0, 1, 4], vec![1, 0, 2], vec![4, 2, 0]];
        assert_eq!(dijkstra(Graph { matrix }, 1), [1, 0, 2]);
    }

    #[test]
    fn test_dijkstra_disconnected_nodes() {
        let matrix = vec![
            vec![0, 5, i32::MAX],
            vec![5, 0, i32::MAX],
            vec![i32::MAX, i32::MAX, 0],
        ];
        assert_eq!(dijkstra(Graph { matrix }, 0), [0, 5, i32::MAX]);
    }

    #[test]
    fn test_dijkstra_five_nodes_complex() {
        let matrix = vec![
            vec![0, 10, i32::MAX, 5, i32::MAX],
            vec![10, 0, 3, i32::MAX, 7],
            vec![i32::MAX, 3, 0, i32::MAX, 1],
            vec![5, i32::MAX, i32::MAX, 0, 2],
            vec![i32::MAX, 7, 1, 2, 0],
        ];
        assert_eq!(dijkstra(Graph { matrix }, 0), [0, 8, 11, 5, 7]);
    }

    #[test]
    fn test_dijkstra_all_same_weight() {
        let matrix = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1],
            vec![1, 1, 1, 0],
        ];
        assert_eq!(dijkstra(Graph { matrix }, 0), [0, 1, 1, 1]);
    }

    #[test]
    fn test_dijkstra_fully_connected() {
        let matrix = vec![
            vec![0, 2, 4, 6],
            vec![2, 0, 1, 3],
            vec![4, 1, 0, 2],
            vec![6, 3, 2, 0],
        ];
        assert_eq!(dijkstra(Graph { matrix }, 0), [0, 2, 3, 5]);
    }
}
