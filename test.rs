pub mod graph {
    pub struct Graph<T> {
        vertices: Vec<T>,
        edges: Vec<(usize, usize)>, 
    }

    impl<T> Graph<T> {
        pub fn new() -> Self {
            Graph { vertices: vec![], edges: vec![] }
        }

        pub fn add_vertex(&mut self, vertex: T) -> usize {
            self.vertices.push(vertex);
            self.vertices.len() - 1
        }
        
        pub fn add_edge(&mut self, start: usize, end: usize) {
            if start >= self.vertices.len() || end >= self.vertices.len() {
                panic!("One or both vertex indices are out of bounds");
            }
            self.edges.push((start, end));
        }        

        pub fn vertex_count(&self) -> usize {
            self.vertices.len()
        }

        pub fn edge_count(&self) -> usize {
            self.edges.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_operations() {
        let mut graph = graph::Graph::<String>::new();
        let v1 = graph.add_vertex("Node1".to_string());
        let v2 = graph.add_vertex("Node2".to_string());
        graph.add_edge(v1, v2);

        assert_eq!(graph.vertex_count(), 2, "There should be 2 vertices in the graph.");
        assert_eq!(graph.edge_count(), 1, "There should be 1 edge in the graph.");
    }
}