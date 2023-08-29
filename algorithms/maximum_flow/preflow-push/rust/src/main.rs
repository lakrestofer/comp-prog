use std::sync::{Arc, Mutex};

fn main() {
    let input: String = std::fs::read_to_string("./data/big/001.in").unwrap();
    let first_n: Vec<&str> = input.lines().take(10).collect();
    println!("input: {:?}", first_n)

    // let network = Network::parse(input);

    // let max_flow = network.max_flow();

    // println!("Maximum flow is: {max_flow}");
}

type NodeId = usize;
type EdgeId = usize;
type EdgePointer = Arc<Mutex<Edge>>;
type NodePointer = Arc<Mutex<Node>>;

#[derive(Default, Clone)]
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
    n: usize, // number of nodes
    m: usize, // number of edges
    sink: NodeId,
    drain: NodeId,
    nodes: Vec<Node>,
    edges: Vec<(usize, usize)>,
    adjacency_matrix: Vec<Vec<Option<EdgePointer>>>,
    adjacency_list: Vec<Vec<EdgePointer>>,
}

impl Network {
    fn parse(input: String) -> Self {
        let mut numbers = input
            .split_whitespace()
            .map(|c| c.parse())
            .filter(|nr| nr.is_ok())
            .map(|nr| nr.unwrap());

        // the number of nodes and edges
        let n = numbers.next().unwrap();
        let m = numbers.next().unwrap();
        // we skip the next two numbers since we are not interested in them
        numbers.next();
        numbers.next();

        let nodes = vec![Node::default(); n];
        let edges = Vec::with_capacity(m);
        let adjacency_matrix = vec![vec![None; n]; n];
        let adjacency_list = vec![Vec::new(); n];

        for _ in 0..m {
            let from = numbers.next();
            let to = numbers.next();
            let capacity = numbers.next();
        }

        Network {
            n,
            m,
            sink: 0,
            drain: n - 1,
            nodes,
            edges,
            adjacency_matrix,
            adjacency_list,
        }
    }
}

impl Network {
    fn max_flow(self) -> usize {
        todo!()
    }
}
