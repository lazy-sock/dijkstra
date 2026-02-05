use crate::graph::Graph;
use crate::graph::get_random_graph;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let graph = get_random_graph();
    println!("{}", graph);
    println!("{:?}", dijkstra(&graph, 0));
}

#[derive(Eq, PartialEq)]
struct State {
    cost: i32,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // reverse for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &Graph, source: usize) -> Vec<i32> {
    let n = graph.matrix.len();
    let mut dist = vec![i32::MAX; n];
    dist[source] = 0;

    let mut pq = BinaryHeap::new();
    pq.push(State {
        cost: 0,
        node: source,
    });

    while let Some(State { cost, node }) = pq.pop() {
        if cost > dist[node] {
            continue; // skip if we've found a better path already
        }

        for (neighbor, &edge_weight) in graph.matrix[node].iter().enumerate() {
            if edge_weight > 0 && edge_weight != i32::MAX {
                let next_cost = dist[node].saturating_add(edge_weight);
                if next_cost < dist[neighbor] {
                    dist[neighbor] = next_cost;
                    pq.push(State {
                        cost: next_cost,
                        node: neighbor,
                    });
                }
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
        assert_eq!(dijkstra(&Graph { matrix }, 0), [0]);
    }

    #[test]
    fn test_dijkstra_two_nodes() {
        let matrix = vec![vec![0, 2], vec![2, 0]];
        assert_eq!(dijkstra(&Graph { matrix }, 0), [0, 2]);
    }

    #[test]
    fn test_dijkstra_three_nodes_linear() {
        let matrix = vec![vec![0, 1, i32::MAX], vec![1, 0, 3], vec![i32::MAX, 3, 0]];
        assert_eq!(dijkstra(&Graph { matrix }, 0), [0, 1, 4]);
    }

    #[test]
    fn test_dijkstra_three_nodes_triangle() {
        let matrix = vec![vec![0, 2, 5], vec![2, 0, 1], vec![5, 1, 0]];
        assert_eq!(dijkstra(&Graph { matrix }, 0), [0, 2, 3]);
    }

    #[test]
    fn test_dijkstra_four_nodes() {
        let matrix = vec![
            vec![0, 4, 2, i32::MAX],
            vec![4, 0, 1, 5],
            vec![2, 1, 0, 8],
            vec![i32::MAX, 5, 8, 0],
        ];
        assert_eq!(dijkstra(&Graph { matrix }, 0), [0, 3, 2, 8]);
    }

    #[test]
    fn test_dijkstra_from_different_source() {
        let matrix = vec![vec![0, 1, 4], vec![1, 0, 2], vec![4, 2, 0]];
        assert_eq!(dijkstra(&Graph { matrix }, 1), [1, 0, 2]);
    }

    #[test]
    fn test_dijkstra_disconnected_nodes() {
        let matrix = vec![
            vec![0, 5, i32::MAX],
            vec![5, 0, i32::MAX],
            vec![i32::MAX, i32::MAX, 0],
        ];
        assert_eq!(dijkstra(&Graph { matrix }, 0), [0, 5, i32::MAX]);
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
        assert_eq!(dijkstra(&Graph { matrix }, 0), [0, 8, 8, 5, 7]);
    }

    #[test]
    fn test_dijkstra_all_same_weight() {
        let matrix = vec![
            vec![0, 1, 1, 1],
            vec![1, 0, 1, 1],
            vec![1, 1, 0, 1],
            vec![1, 1, 1, 0],
        ];
        assert_eq!(dijkstra(&Graph { matrix }, 0), [0, 1, 1, 1]);
    }

    #[test]
    fn test_dijkstra_fully_connected() {
        let matrix = vec![
            vec![0, 2, 4, 6],
            vec![2, 0, 1, 3],
            vec![4, 1, 0, 2],
            vec![6, 3, 2, 0],
        ];
        assert_eq!(dijkstra(&Graph { matrix }, 0), [0, 2, 3, 5]);
    }
}
