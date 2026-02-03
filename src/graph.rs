use std::fmt;

pub struct Graph {
    matrix: Vec<Vec<i32>>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut matrix = String::from("");
        for i in &self.matrix {
            let mut row = String::from("");
            for j in i {
                row += &j.to_string();
            }
            row += "\n";
            matrix += &row;
        }
        write!(f, "{}", matrix)
    }
}

pub fn get_random_graph() -> Graph {
    let size = rand::random_range(0..10);
    let mut matrix = vec![];
    for _ in 0..size {
        let mut row = vec![];
        for _ in 0..size {
            row.push(rand::random_range(0..10));
        }
        matrix.push(row);
    }
    Graph { matrix }
}
