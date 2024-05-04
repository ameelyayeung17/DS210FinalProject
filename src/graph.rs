use std::collections::HashMap;

pub struct Graph<T> where T: PartialEq + Eq {  
    pub vertices: Vec<T>,
    pub edges: Vec<(usize, usize)>,
    adj_list: HashMap<usize, Vec<usize>>,
}

impl<T: PartialEq + Eq> Graph<T> {  
    pub fn new() -> Self {
        Self { 
            vertices: vec![], 
            edges: vec![], 
            adj_list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: T) -> usize {
        if let Some(index) = self.vertices.iter().position(|v| *v == vertex) {
            index 
        } else {
            let index = self.vertices.len();
            self.vertices.push(vertex);
            self.adj_list.entry(index).or_insert_with(Vec::new);
            index
        }
    }

    pub fn add_edge(&mut self, start: usize, end: usize) {
        self.edges.push((start, end));
        self.adj_list.entry(start).or_default().push(end);  
        self.adj_list.entry(end).or_default().push(start);  
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn neighbors(&self, vertex: usize) -> Vec<usize> {
        self.adj_list.get(&vertex).cloned().unwrap_or_else(Vec::new)
    }
}