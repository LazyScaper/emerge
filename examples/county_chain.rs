use emerge::graph::{default_window_conf, render_graph, Graph};

#[macroquad::main(default_window_conf)]
async fn main() {
    let mut graph = Graph::new();

    graph.add_node("Albania");
    graph.add_node("Cambodia");
    graph.add_node("Cameroon");
    graph.add_node("Nigeria");

    graph.add_directed_edge("Cambodia", "Albania");
    graph.add_directed_edge("Cameroon", "Nigeria");
    graph.add_directed_edge("Nigeria", "Albania");

    render_graph(graph).await;
}
