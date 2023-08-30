use std::sync::{Arc, Mutex};

fn main() {
    // let input: String = std::fs::read_to_string("./data/big/001.in").unwrap();
    // let input: String = std::fs::read_to_string("./data/railwayplanning/secret/2med.in").unwrap();
    // let input: String = std::fs::read_to_string("./data/railwayplanning/secret/1small.in").unwrap();
    let input: String = std::fs::read_to_string("./data/tiny/0.in").unwrap();
    let first_n: Vec<&str> = input.lines().take(10).collect();
    println!("input: {:?}", first_n);

    let network = Network::parse(input);
    println!("graph: {:?}", network.adjacency_list);

    // let max_flow = network.max_flow();

    // println!("Maximum flow is: {max_flow}");
}

type NodeId = usize;
type EdgeId = usize;
type EdgePointer = Arc<Mutex<Edge>>;
type NodePointer = Arc<Mutex<Node>>;

#[derive(Default, Clone, Debug)]
struct Node {
    excess_flow: i64,
    height: usize,
}

#[derive(Default, Clone, Debug)]
struct Edge {
    pub max_capacity: i64,
    pub flow: i64,
    pub from: NodeId,
    pub to: NodeId,
    pub forward_edge: bool,
}

/// our graph datastructure
#[derive(Debug)]
struct Network {
    n: usize, // number of nodes
    m: usize, // number of edges
    sink: NodeId,
    drain: NodeId,
    nodes: Vec<Node>,
    adjacency_list: Vec<Vec<EdgePointer>>,
    adjacency_matrix: Vec<Vec<Option<EdgePointer>>>,
}

impl Network {
    fn parse(input: String) -> Self {
        let mut numbers = input
            .split_whitespace()
            .map(|c| c.parse::<usize>())
            .filter(|nr| nr.is_ok())
            .map(|nr| nr.unwrap());

        // the number of nodes and edges
        let n = numbers.next().unwrap();
        let m = numbers.next().unwrap();
        // we skip the next two numbers since we are not interested in them
        numbers.next();
        numbers.next();

        let nodes = vec![Node::default(); n];
        let mut adjacency_list: Vec<Vec<EdgePointer>> = vec![Vec::new(); n];
        let mut adjacency_matrix: Vec<Vec<Option<EdgePointer>>> = vec![vec![None; n]; n];

        for _ in 0..m {
            let from = numbers.next().unwrap();
            let to = numbers.next().unwrap();
            let max_capacity = numbers.next().unwrap() as i64;
            let flow = 0;

            let forward_edge = Arc::new(Mutex::new(Edge {
                max_capacity,
                flow,
                from,
                to,
                forward_edge: true,
            }));

            let backward_edge = Arc::new(Mutex::new(Edge {
                max_capacity,
                flow,
                from: to,
                to: from,
                forward_edge: false,
            }));

            // The adjacency list and matrix will contain all leaving and entering edges.
            // Us using Arc<Mutex<T>> (Rc<Cell> would have worked as well) makes it possible to use the O(1) retreiveal of neighbors
            // and specific edges.
            adjacency_list[from].push(forward_edge.clone()); // we save the forward edge in both the adjacency list
            adjacency_matrix[from][to] = Some(forward_edge); // and matrix
            adjacency_list[to].push(backward_edge.clone()); // and then the bakwards edge
            adjacency_matrix[to][from] = Some(backward_edge);
        }

        Network {
            n,
            m,
            sink: 0,
            drain: n - 1,
            nodes,
            adjacency_list,
            adjacency_matrix,
        }
    }
}

impl Network {
    fn max_flow(mut self) -> i64 {
        // set the excess flow of all nodes to 0 (default)
        // set the height of every other node to 0 (default)
        // we do nothing...

        // set the height of the source to the number of nodes
        self.nodes[0].height = self.n;

        // we keep track of all nodes whose effective flow is positive (meaning more flow in than out)
        let mut overflowing_nodes: Vec<usize> = Vec::new();

        // the outward flow from the source is the maximum given by the edges capacity
        for edge in self.adjacency_list[0].iter_mut() {
            let mut edge = edge.lock().unwrap();
            let edge_capacity = edge.max_capacity;
            // the flow is set to its maximum for each edge
            edge.flow = edge_capacity;
            // we set the effective flow of the target to the incomming flow
            self.nodes[edge.to].excess_flow = edge_capacity;
            // this node will have positive excess flow so we add it to the overflowing_nodes list
            overflowing_nodes.push(edge.to);
            self.nodes[0].excess_flow -= edge_capacity;
        }

        // set the flow of every other edge that do not start in the source to zero (default)
        // we do nothing

        while !overflowing_nodes.is_empty() {
            let from_node = overflowing_nodes.pop().unwrap();
            // if this node is the tail or has been updated to not overflow
            // since it got put into the stack we skip this iteration
            if from_node == self.n - 1 || self.nodes[from_node].excess_flow <= 0 {
                continue;
            };

            // we see if there exists an edge
            let edge = self.adjacency_list[from_node]
                .iter()
                .find(|edge| {
                    let edge = edge.lock().unwrap();
                    self.nodes[edge.from].height > self.nodes[edge.to].height
                })
                .map(|e| e.clone());

            if let Some(edge) = edge {
                let edge = edge.lock().unwrap();
                self.push(edge.from, edge.to);
            } else {
                self.relabel(from_node)
            }
        }

        // the maximum flow is the effective flow of the drain
        let maximum_flow = self.nodes[self.n - 1].excess_flow;
        maximum_flow
    }

    fn push(&mut self, from: usize, to: usize) {
        // check that the edge is
        assert!(
            self.nodes[from].excess_flow > 0 && (self.nodes[from].height > self.nodes[to].height)
        );

        let mut edge = self.adjacency_matrix[from][to]
            .as_ref()
            .unwrap()
            .lock()
            .unwrap();

        if edge.forward_edge {
            let excess_flow = self.nodes[from].excess_flow;
            let delta = excess_flow.min(edge.max_capacity - edge.flow);
            edge.flow += delta;
            self.nodes[edge.from].excess_flow -= delta;
            self.nodes[edge.to].excess_flow += delta;
        } else {
            // this edge is not a forward edge, we still want to modify the forward edge though, so we have to retrieve it.
            let mut edge = self.adjacency_matrix[to][from]
                .as_ref()
                .unwrap()
                .lock()
                .unwrap();

            let delta = self.nodes[edge.from].excess_flow.min(edge.flow);

            edge.flow -= delta;
            self.nodes[edge.from].excess_flow += delta;
            self.nodes[edge.to].excess_flow -= delta;
        }
    }

    fn relabel(&mut self, node: usize) {}
}
