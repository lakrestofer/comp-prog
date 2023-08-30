use std::sync::{Arc, Mutex};

fn main() {
    // let input: String = std::fs::read_to_string("./data/big/001.in").unwrap();
    // let input: String = std::fs::read_to_string("./data/railwayplanning/secret/2med.in").unwrap();
    // let input: String = std::fs::read_to_string("./data/railwayplanning/secret/1small.in").unwrap();
    let input: String = std::fs::read_to_string("./data/tiny/0.in").unwrap();
    let first_n: Vec<&str> = input.lines().take(10).collect();
    println!("input: {:?}\n", first_n);

    let mut network = new_network(input);

    let max_flow = max_flow(&mut network);

    println!("Maximum flow is: {max_flow}");
}

fn print_network(network: &Network) {
    println!("Network: with {} nodes and {} edges", network.n, network.m);
    println!("nodes");
    for node in network.nodes.iter() {
        println!("{:?}", node);
    }
    for edge in network.edges.iter() {
        let edge = edge.lock().unwrap();
        println!("{:?}", edge);
    }
    print!("Overflowing nodes: {:?}", network.overflowing_nodes);
    println!("\n");
}

type NodeId = usize;
type EdgePointer = Arc<Mutex<Edge>>;

#[derive(Default, Clone, Debug)]
struct Node {
    id: NodeId,
    e: i64,
    h: usize,
}

impl Node {
    fn new(id: NodeId) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }
}

#[derive(Default, Clone, Debug)]
struct Edge {
    /// the source node
    pub u: NodeId,
    /// the destination node
    pub v: NodeId,
    /// maximum flow capacity
    pub c: i64,
    /// the current flow
    pub f: i64,
}

fn new_edge(u: NodeId, v: NodeId, c: i64, f: i64) -> Arc<Mutex<Edge>> {
    Arc::new(Mutex::new(Edge { u, v, c, f }))
}

/// our graph datastructure
#[derive(Debug)]
struct Network {
    /// number of nodes
    n: usize,
    /// number of edges
    m: usize,
    /// the source
    source: NodeId,
    /// the drain (sink)
    drain: NodeId,
    /// nodes
    nodes: Vec<Node>,
    /// edges
    edges: Vec<EdgePointer>,
    /// adjacency list
    adjacency_list: Vec<Vec<EdgePointer>>,
    /// overflowing nodes with positive excess flow (meaning more flow in than out)
    overflowing_nodes: Vec<NodeId>,
}

fn new_network(input: String) -> Network {
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

    // adjacency list
    let mut al: Vec<Vec<EdgePointer>> = vec![Vec::new(); n];
    let mut edges = Vec::with_capacity(m);

    for _ in 0..m {
        let u = numbers.next().unwrap();
        let v = numbers.next().unwrap();
        let c = numbers.next().unwrap() as i64;
        let f = 0;

        let edge = new_edge(u, v, c, f);
        edges.push(edge.clone());

        al[u].push(edge.clone()); // we save the forward edge in both the adjacency list
        al[v].push(edge.clone()); // and then the backwards edge
    }

    let mut nodes = Vec::with_capacity(n);

    for i in 0..n {
        nodes.push(Node::new(i));
    }

    Network {
        n,
        m,
        source: 0,
        drain: n - 1,
        nodes,
        edges,
        adjacency_list: al,
        overflowing_nodes: Vec::<NodeId>::new(),
    }
}

fn max_flow(network: &mut Network) -> i64 {
    println!("before init: ");
    print_network(network);

    let s_id = network.source;
    // set the height of the source to the number of nodes
    network.nodes[s_id].h = network.n;
    // set the height of every other node to 0 (default)

    // the outward flow from the source is the maximum given by the edges capacity
    for u_v in network.adjacency_list[s_id].clone().iter() {
        let mut u_v = u_v.lock().unwrap();

        // set the excess flow of the source node
        network.nodes[s_id].e += u_v.c; // TODO

        push(
            network,
            s_id,
            // the graph might have a edge backward edge from s to v
            if s_id == u_v.u { u_v.v } else { u_v.u },
            &mut u_v,
        );
    }
    println!("after init: ");
    print_network(network);

    // set the excess flow of all nodes to 0 (default)

    // set the flow of every other edge that do not start in the source to zero (default)
    // we do nothing

    while !network.overflowing_nodes.is_empty() {
        print_network(network);
        let u: NodeId = network.overflowing_nodes.pop().unwrap();
        println!("u: {u}");
        // if this node is the tail or has been updated to not overflow
        // since it got put into the stack we skip this iteration
        if u == network.n - 1 || network.nodes[u].e <= 0 {
            println!("continuing");
            continue;
        };

        // we see if there exists an edge
        let neighbors = &network.adjacency_list[u];
        println!("neighbors: {:?}", neighbors);
        let u_v = neighbors
            .iter()
            .find(|v_w| {
                let edge = v_w.lock().unwrap();
                edge.u == u && network.nodes[edge.u].h > network.nodes[edge.v].h
            })
            .map(|e| e.clone());

        if let Some(u_v) = u_v {
            let mut u_v = u_v.lock().unwrap();

            push(network, u_v.u, u_v.v, &mut u_v);
        } else {
            relabel(network, u);
        }
    }

    // the maximum flow is the effective flow of the drain
    network.nodes[network.drain].e
}

fn push(network: &mut Network, u: NodeId, v: NodeId, e: &mut Edge) {
    let d;
    if u == e.u {
        d = network.nodes[u].e.min(e.c - e.f);
        e.f += d;
    } else {
        d = network.nodes[u].e.min(e.c + e.f);
        e.f -= d;
    }

    network.nodes[u].e -= d;
    network.nodes[v].e += d;

    if cfg!(debug_assertions) {
        assert!(d >= 0);
        assert!(network.nodes[u].e >= 0);
        assert!(e.f.abs() <= e.c);
    }

    if network.nodes[u].e > 0 && network.source != u && network.drain != u {
        network.overflowing_nodes.push(u);
    }

    if network.nodes[v].e == d && network.source != v && network.drain != v {
        network.overflowing_nodes.push(v);
    }
}

fn relabel(network: &mut Network, u: NodeId) {
    network.nodes[u].h += 1;

    if network.source != u && network.drain != u {
        network.overflowing_nodes.push(u);
    }
}
