use emerge::graph::{default_window_conf, render_graph, Graph};

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
