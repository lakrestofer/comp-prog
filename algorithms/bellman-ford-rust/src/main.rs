fn main() {
    let mut graph = Graph::new();


}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new()
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node)
    }
    pub fn add_edge(&mut self,) -> bool {

    }
}

struct Edge(usize, usize);
struct Node {
    elem: String
}