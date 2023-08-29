use std::sync::{Arc, Mutex};

fn main() {
    let input = String::new();

    let network = Network::parse(input);

    let max_flow = network.max_flow();

    println!("Maximum flow is: {max_flow}");
}

type NodeId = usize;
type EdgeId = usize;
type EdgePointer = Arc<Mutex<Edge>>;
type NodePointer = Arc<Mutex<Node>>;

struct Node {
    incomming_flow: usize,
    outgoing_flow: usize,
}

struct Edge {
    pub max_capacity: usize,
    pub flow: usize,
    pub from: NodeId,
    pub to: NodeId,
}

/// our graph datastructure
struct Network {
    sink: NodeId,
    drain: NodeId,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    adjacency_matrix: Vec<Vec<Option<EdgePointer>>>,
    adjacency_list: Vec<Vec<EdgePointer>>,
}

impl Network {
    fn parse(input: String) -> Self {
        todo!()
    }
}

impl Network {
    fn max_flow(self) -> usize {
        todo!()
    }
}
