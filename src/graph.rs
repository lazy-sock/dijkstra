use rand::prelude::*;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub struct Graph {
    value: i32,
    nodes: RefCell<Vec<Rc<Edge>>>,
}

pub struct Edge {
    start: Rc<Graph>,
    end: Rc<Graph>,
    value: i32,
}

impl Graph {
    fn get_nodes(&self) -> Vec<&Graph> {
        let mut nodes = vec![self];
        for i in self.nodes.borrow().iter() {
            for j in i.end.get_nodes() {
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

pub fn get_random_graph(root_value: Option<i32>) -> Rc<Graph> {
    // root_value is an optional parameter
    let root = Graph {
        value: root_value.unwrap_or(42),
        nodes: RefCell::from(vec![]),
    };

    let number_of_nodes = rand::random_range(0..10);

    let root_rc = Rc::new(root);

    for _ in 0..number_of_nodes {
        let mut node = Rc::from(Graph {
            value: rand::random_range(0..1000),
            nodes: RefCell::from(vec![]),
        });
        let has_children = rand::random_bool(0.2);
        if has_children {
            node = get_random_graph(Some(node.value)); // can sometimes produce stack overflow
        }
        root_rc.nodes.borrow_mut().push(Rc::from(Edge {
            start: Rc::clone(&root_rc),
            end: Rc::from(node),
            value: 0,
        }));
    }

    root_rc
}
