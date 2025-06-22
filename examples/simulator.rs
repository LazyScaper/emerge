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

        self.number_of_edges = rng.random_range(1..300);
        self.number_of_nodes = rng.random_range(1..50);

        let mut graph = Graph::new();

        for node_id in 0..self.number_of_nodes {
            graph.add_node("node".to_string() + node_id.to_string().as_str());
        }

        for _ in 0..self.number_of_edges {
            graph.add_edge(
                rng.random_range(..self.number_of_nodes),
                rng.random_range(..self.number_of_nodes),
            );
        }

        graph
    }
}

#[macroquad::main(default_window_conf)]
async fn main() {
    let graph = Simulator::new().graph();

    render_graph(graph).await;
}
