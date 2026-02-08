// Goal: Create a random generated Graph using petgraph
use petgraph::graph::DiGraph;

pub fn generate_random_graph() {
    let graph: DiGraph<u32, u32> = DiGraph::from_edges(&[(0, 1), (1, 2), (2, 3), (0, 3)]);
    dbg!(graph);
}

#[cfg(test)]
#[test]
fn debug() {
    generate_random_graph();
}
