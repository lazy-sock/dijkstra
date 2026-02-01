use crate::graph::get_random_graph;

fn main() {
    println!("Hello, world!");
    let graph = get_random_graph();
    println!("{}", graph);
}

mod graph;
