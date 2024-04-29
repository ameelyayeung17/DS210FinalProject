use graphlib:: {Graph, VertexId};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn load_graph_data(graph: &mut Graph<String>, filename: &str) {
    let path = Path::new(filename);
    let file = File::open(path).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Unable to read");
        let nodes: Vec<&str> = line.split_whitespace().collect();
        if nodes.len() == 2 {
            let vertex1 = graph.add_vertex(nodes[0].to_string());
            let vertex2 = graph.add_vertex(nodes[1].to_string());
            let _ = graph.add_edge(&vertex1, &vertex2);
        }
    
    }
}
fn bfs_distances(graph: &Graph<String>, start: VertexId) -> HashMap<VertexId, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    distances.insert(start, 0);
    queue.push_back(start);
    while let Some(current) = queue.pop_front() {
        let current_distance = distances[&current];
        for neighbor in graph.neighbors(&current) {
            if !distances.contains_key(&neighbor) {
                distances.insert(*neighbor, current_distance + 1);
                queue.push_back(*neighbor);
            }
        }
    }
    distances
}
fn calculate_average_path_length(graph: &Graph<String>) -> f64 {
    let mut total_distance = 0;
    let mut path_count = 0;

    for vertex in graph.vertices() { 
        let distances = bfs_distances(graph, vertex.clone());
        for &distance in distances.values() {
            if distance > 0 {
                total_distance += distance;
                path_count += 1;
        
            }
        }
    }
    if path_count > 0 {
        total_distance as f64 / path_count as f64 
    } else {
        0.0
    }
}
fn main() {
    let mut graph = Graph::<String>::new();
    load_graph_data(&mut graph, "facebook_combined.txt");
    let avg_path_length = calculate_average_path_length(&graph);
    println!("Average path length in the graph: {}", avg_path_length);
}