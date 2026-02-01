use std::fmt;
use std::rc::Rc;

pub struct Graph {
    value: i32,
    nodes: Vec<Rc<Graph>>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub fn get_random_graph() -> Graph {
    let mut root = Graph {
        value: 42,
        nodes: vec![],
    };

    for i in 0..5 {
        let node = Graph {
            value: i,
            nodes: vec![],
        };
        root.nodes.push(Rc::from(node));
    }

    root
}
