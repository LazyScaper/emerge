use emerge::graph::{render_graph, Graph};
use macroquad::prelude::Conf;

fn window_conf() -> Conf {
    Conf {
        window_title: "Emerge - Graph".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut graph = Graph::new();

    graph.add_node("Albania".to_string(), "Albania".to_string());
    graph.add_node("Cambodia".to_string(), "Cambodia".to_string());
    graph.add_node("Cameroon".to_string(), "Cameroon".to_string());
    graph.add_node("Nigeria".to_string(), "Nigeria".to_string());

    graph.add_edge_by_name("Cambodia", "Albania");
    graph.add_edge_by_name("Cameroon", "Nigeria");
    graph.add_edge_by_name("Nigeria", "Albania");

    render_graph(graph).await;
}
