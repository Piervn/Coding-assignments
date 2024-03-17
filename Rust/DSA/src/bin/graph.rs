/// Undirected graph as a list of edges:
/// `(u <-> v)` 
pub struct Graph {
    edges: Vec<(u32, u32)>,
    vertices: u32
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: vec![],
            vertices: 0
        }
    }

    pub fn from_vec(edges: Vec<(u32, u32)>) -> Self {
        let mut graph = Self::new();
        for (u, v) in edges {
            graph.add_edge(u, v);
        }
        graph
    }

    pub fn edges(&self) -> &Vec<(u32, u32)> {
        return &self.edges;
    }

    pub fn vertices(&self) -> u32 {
        return self.vertices;
    }
    
    pub fn add_edge(&mut self, u: u32, v: u32) {
        self.edges.push((u, v));
        self.vertices = self.vertices.max(u + 1).max(v + 1);
    }
} 

/// Directed graph as a list of edges:
/// `(u -> v)`
pub struct DiGraph {
    graph: Graph
}

impl DiGraph {
    pub fn new() -> Self {
        Self {
            graph: Graph::new()
        }
    }

    pub fn from_vec(edges: Vec<(u32, u32)>) -> Self {
        Self {
            graph: Graph::from_vec(edges)
        }
    }

    pub fn edges(&self) -> &Vec<(u32, u32)> {
        return &self.graph.edges;
    }

    pub fn vertices(&self) -> u32 {
        return self.graph.vertices();
    }

    pub fn add_edge(&mut self, u: u32, v: u32) {
        self.graph.add_edge(u, v);
    }

    pub fn to_adjacency_list(&self) -> Vec<Vec<u32>> {
        let mut adjacency_list = vec![vec![]; self.graph.vertices() as usize];
        for (u, v) in self.graph.edges() {
            adjacency_list[*u as usize].push(*v);
        }
        return adjacency_list;
    }

    pub fn to_rev_adjacency_list(&self) -> Vec<Vec<u32>> {
        let mut in_adjacency_list = vec![vec![]; self.graph.vertices() as usize];
        for (u, v) in self.graph.edges() {
            in_adjacency_list[*v as usize].push(*u);
        }
        return in_adjacency_list;
    }
}   

fn main() {}