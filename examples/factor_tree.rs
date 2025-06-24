use emerge::graph::{default_window_conf, render_graph, Graph};

fn build_factor_tree(value: i32) -> Graph {
    let mut graph = Graph::new();

    compute_factor_tree(value, &mut graph);

    graph
}

fn compute_factor_tree(value: i32, graph: &mut Graph) {
    if value == 1 {
        return;
    }

    graph.add_node(&value.to_string());

    for number_to_check_as_factor in (2..(value as f32).sqrt() as i32 + 1).rev() {
        if value % number_to_check_as_factor == 0 {
            let first_factor = number_to_check_as_factor;
            let second_factor = (value / number_to_check_as_factor);

            graph.add_node(&first_factor.to_string());
            graph.add_node(&second_factor.to_string());
            graph.add_directed_edge(&value.to_string(), &first_factor.to_string());
            graph.add_directed_edge(&value.to_string(), &second_factor.to_string());

            compute_factor_tree(first_factor, graph);
            compute_factor_tree(second_factor, graph);
            return;
        }
    }
}

#[macroquad::main(default_window_conf)]
async fn main() {
    let graph = build_factor_tree(456);

    render_graph(graph).await;
}
