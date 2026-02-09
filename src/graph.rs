// Goal: Create a random generated Graph using petgraph
use petgraph::graph::DiGraph;

// To generate a random Graph the Configuration model (https://en.wikipedia.org/wiki/Configuration_model) seems appropiate
pub fn generate_random_graph() {
    // We first need a random degree sequence
    let n = rand::random_range(0..10);
    let mut degree: Vec<u32> = vec![];
    for _ in 0..n {
        degree.push(rand::random_range(0..10));
    }

    let edges = [(0, 1, 2), (1, 2, 5), (2, 3, 1), (0, 3, 2)];
    let graph: DiGraph<u32, u32> = DiGraph::from_edges(edges);
    dbg!(graph);
}

#[cfg(test)]
#[test]
fn debug() {
    generate_random_graph();
}
