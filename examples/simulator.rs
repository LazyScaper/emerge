use emerge::graph::{default_window_conf, render_graph, Graph};
use random::Rng;

struct Simulator {
    number_of_nodes: usize,
    number_of_edges: usize,
}

impl Simulator {
    fn new() -> Self {
        Self {
            number_of_nodes: 30,
            number_of_edges: 30,
        }
    }

    fn graph(&mut self) -> Graph {
        let mut rng = random::rng();

        self.number_of_edges = rng.random_range(..100);
        self.number_of_nodes = rng.random_range(..100);

        let graph = Graph::new();

        for _ in 0..self.number_of_nodes {
            // graph.add_node("Node".to_string(), "Node".to_string());
        }
        graph
    }
}

#[macroquad::main(default_window_conf)]
async fn main() {
    let mut graph = Graph::new();

    graph.add_node("Albania".to_string());
    graph.add_node("Cambodia".to_string());
    graph.add_node("Cameroon".to_string());
    graph.add_node("Nigeria".to_string());

    graph.add_edge_by_name("Cambodia", "Albania");
    graph.add_edge_by_name("Cameroon", "Nigeria");
    graph.add_edge_by_name("Nigeria", "Albania");

    render_graph(graph).await;
}
