use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        graph: [[Usize1; 2]; n],
    }
    let mut depth = vec![1 << 30; n];
    breadth_first_search(&graph, &mut depth, 0);
    let mut u = 0;
    let m = depth.iter().max().unwrap();
    for i in 0..n {
        if &depth[i] == m {
            u == i;
        }
    }
    let mut depth = vec![1 << 30; n];
    breadth_first_search(&graph, &mut depth, u);
    println!("{}", 1 + depth.iter().max().unwrap());
}

fn breadth_first_search(graph: &Vec<Vec<usize>>, depth: &mut Vec<isize>, start: usize) {
    let mut queue: VecDeque<usize> = VecDeque::new();
    depth[start] = 0;
    queue.push_back(start);
    while queue.len() > 0 {
        let i = queue.pop_front().unwrap();
        for j in graph[i].iter() {
            if depth[*j] == 1 << 30 {
                queue.push_back(*j);
                depth[*j] = depth[i] + 1;
            }
        }
    }
}
