use std::collections::{HashMap, VecDeque};
use crate::graph::Graph; 

pub fn bfs_distances(graph: &Graph<String>, start: usize) -> HashMap<usize, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    distances.insert(start, 0);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        let current_distance = *distances.get(&current).unwrap(); 
        for neighbor in graph.neighbors(current) {
            if !distances.contains_key(&neighbor) {
                distances.insert(neighbor, current_distance + 1);
                queue.push_back(neighbor);
            }
        }
    }
    distances
}

pub fn calculate_average_path_length(graph: &Graph<String>) -> f64 {
    let mut total_distance = 0;
    let mut path_count = 0;

    for vertex_index in 0..graph.vertex_count() {
        let distances = bfs_distances(graph, vertex_index);
        for &distance in distances.values() {
            if distance > 0 {
                total_distance += distance;
                path_count += 1;
            }
        }
    }
    if path_count > 0 {
        let average_path_length = total_distance as f64 / path_count as f64;
        println!("Final average path length: {}", average_path_length); 
        average_path_length
    } else {
        println!("No paths found, returning 0.0"); 
        0.0
    }
}