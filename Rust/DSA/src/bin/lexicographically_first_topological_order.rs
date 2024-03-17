mod graph;
use graph::DiGraph;
mod heap;
use heap::{Heap, HeapType};

fn first_order(graph: &DiGraph) -> Vec<u32> {
    let mut order = vec![];
    let mut indegrees = vec![0; graph.vertices() as usize]; 
    for (u, v) in graph.edges() {
        indegrees[*v as usize] += 1;
    }
    let adj_list = graph.to_adjacency_list();
    let mut queue = Heap::<u32>::new(HeapType::MIN);

    for i in 0..graph.vertices() {
        if indegrees[i as usize] == 0 {
            queue.push(i);
        }
    }

    while queue.len() > 0 {
        let u = queue.pop().unwrap();
        order.push(u);
        for v in &adj_list[u as usize] {
            indegrees[*v as usize] -= 1;
            if indegrees[*v as usize] == 0 {
                queue.push(*v);
            }
        }      
    }
    return order;
}

fn main() {

}