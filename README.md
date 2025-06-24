## Emerge

A Rust library for force-directed graphs. Uses physics simulations to let clear, stable network structures emerge naturally.

![emerge - force directed graph.gif](images/emerge%20-%20force%20directed%20graph.gif)

### Features

- 2D directed graph rendering
- Natural node position due to physics based simulation
- Click and drag to navigate around the graph

### Usage

```rust
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
```

### Examples

- [Factor Tree](examples/factor_tree.rs)
- [Simulator](examples/simulator.rs)
- [Country Chain (wip)](examples/county_chain.rs)