use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;   
use crate::graph::Graph; 

pub fn load_graph_data(graph: &mut Graph<String>, filename: &str) {
    let path = Path::new(filename);
    let file = File::open(path).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let nodes: Vec<&str> = line.split_whitespace().collect();
        if nodes.len() == 2 {
            let vertex1 = graph.add_vertex(nodes[0].to_string());
            let vertex2 = graph.add_vertex(nodes[1].to_string());
            graph.add_edge(vertex1, vertex2);
        }
    }
}
