#![allow(dead_code)]

use std::vec;

trait Graph {
    type Node;
    type Edge;

    fn neibors(node: &Self::Node) -> Vec<Self::Edge>;
}

struct  MyGraph {}

impl Graph for MyGraph {
    type Node = String;
    type Edge = (String, String);

    fn neibors(node: &Self::Node) -> Vec<Self::Edge> {
        // ここでは単純な例として、ノードの隣接ノードを固定で返す
        let edges = vec![
            (node.clone(), "A".to_string()),
            (node.clone(), "B".to_string()),
            (node.clone(), "C".to_string()),
        ];
        edges
    }


} 

fn main() {
    let node = "node".to_string();
    let edges = MyGraph::neibors(&node);
    for edge in edges {
        println!("Edge: {:?}", edge);
    }
}
