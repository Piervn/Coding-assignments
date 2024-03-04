/*
Assumptions:
- graph is passed as adjacency list
- each element in list is a tuple (node, weight)
- weights of edges must be non-negative
- nodes are numbered from 0 to n-1
 */

mod heap;
use heap::{Heap, HeapType};

fn dijkstra(graph: &Vec<Vec<(u32, u32)>>, source: u32) -> Vec<u32> {
    if source >= graph.len() as u32 {
        panic!("Invalid source")
    }
    let inf = u32::MAX;
    let mut d: Vec<u32> = vec![inf; graph.len()];
    d[source as usize] = 0;

    let d_for_queue = d
        .iter()
        .enumerate()
        .map(|(i, x)| (x.clone(), i as u32))
        .collect();
    let mut queue: Heap<(u32, u32)> = Heap::from_vec(HeapType::MIN, d_for_queue);
    queue.push((0, source));

    while !queue.empty() {
        let u = queue.pop().unwrap().1;
        for (v, weight) in &graph[u as usize] {
            let alt = d[u as usize] + weight;
            if alt < d[*v as usize] {
                d[*v as usize] = alt;
                let id = queue.find(|(x, _)| *x == *v);
                if id.is_some() {
                    queue.update(id.unwrap(), (alt, *v));
                }
            }
        }
    }
    return d;
}

fn main() {
    println!("Dijkstra algorithm");
}
