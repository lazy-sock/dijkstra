use std::fmt;
use std::rc::Rc;

pub struct Graph {
    value: i32,
    nodes: Vec<Rc<Graph>>,
}

impl Graph {
    fn get_nodes(&self) -> Vec<&Graph> {
        let mut nodes = vec![self];
        for i in &self.nodes {
            for j in i.get_nodes() {
                nodes.push(j);
            }
        }
        nodes
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nodes = self.get_nodes();
        let mut values = vec![];
        for node in nodes {
            values.push(node.value);
        }
        write!(f, "{:?}", values)
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
