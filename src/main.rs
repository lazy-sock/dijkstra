use crate::graph::get_random_graph;

fn main() {
    let graph = get_random_graph(None);
    println!("{}", graph);
}

mod graph;
