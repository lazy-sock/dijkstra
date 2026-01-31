use std::rc::Rc;

struct Graph {
    value: i32,
    nodes: Vec<Rc<Graph>>,
}

fn get_random_graph() -> Graph {
    let mut root = Graph {
        value: 42,
        nodes: vec![],
    };

    root
}
