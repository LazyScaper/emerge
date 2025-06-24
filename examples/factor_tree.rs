use emerge::graph::{default_window_conf, render_graph, Graph};

fn build_factor_tree(value: i32) -> Graph {
    let mut graph = Graph::new();

    let mut visited_nodes = vec![];

    graph.add_node(&value.to_string());

    compute_factor_tree(&mut visited_nodes, value, &mut graph);

    graph
}

fn compute_factor_tree(visited_nodes: &mut Vec<i32>, value: i32, graph: &mut Graph) {
    if value == 1 || visited_nodes.contains(&value) {
        return;
    }

    for number_to_check_as_factor in (2..(value as f32).sqrt() as i32 + 1).rev() {
        if value % number_to_check_as_factor == 0 {
            let first_factor = number_to_check_as_factor;
            let second_factor = (value / number_to_check_as_factor);

            visited_nodes.push(value);

            if first_factor != second_factor {
                graph.add_node(&first_factor.to_string());
                graph.add_node(&second_factor.to_string());
                graph.add_directed_edge(&value.to_string(), &first_factor.to_string());
                graph.add_directed_edge(&value.to_string(), &second_factor.to_string());

                compute_factor_tree(visited_nodes, first_factor, graph);
                compute_factor_tree(visited_nodes, second_factor, graph);
            } else {
                graph.add_node(&first_factor.to_string());
                graph.add_directed_edge(&value.to_string(), &first_factor.to_string());

                compute_factor_tree(visited_nodes, first_factor, graph);
            }
            return;
        }
    }
}

#[macroquad::main(default_window_conf)]
async fn main() {
    let graph = build_factor_tree(34280);

    render_graph(graph).await;
}
