mod graph;
mod io;
mod analysis;

use graph::Graph; 
// calling the different modules 
fn main() {
    let mut graph = Graph::<String>::new();
    io::load_graph_data(&mut graph, "facebook_combined.txt");
    let avg_path_length: f64 = analysis::calculate_average_path_length(&graph);
    println!("Average path length in the graph: {}", avg_path_length);
}