/*
Assumptions:
- graph is passed as adjacency list
- each element in list is a tuple (node, weight)
- weights of edges must be non-negative
- nodes are numbered from 0 to n-1
 */

mod heap;
use heap::{Heap, HeapType};

// handle on swap to boost complexity
fn dijkstra(graph: &Vec<Vec<(u32, u32)>>, source: u32) -> Vec<u32> {
    if source >= graph.len() as u32 {
        panic!("Invalid source")
    }
    let inf = u32::MAX;
    let mut d: Vec<u32> = vec![inf; graph.len()];
    d[source as usize] = 0;

    let mut queue: Heap<(u32, u32)> = Heap::new(HeapType::MIN);
    queue.push((0, source));

    while !queue.empty() {
        let (dist, u) = queue.pop().unwrap();
        if d[u as usize] < dist {
            continue;
        }
        for (v, weight) in &graph[u as usize] {
            let alt = d[u as usize].saturating_add(*weight);
            if alt < d[*v as usize] {
                d[*v as usize] = alt;
            }
            queue.push((alt, *v));
        }
    }
    return d;
}

fn main() {
    let graph = vec![
        vec![],
        vec![(0, 2), (2, 3), (3, 4), (4, 1)],
        vec![(0, 3)],
        vec![],
        vec![(3, 2)],
    ];
    let d = dijkstra(&graph, 1);
    println!("{:?}", d);
}
