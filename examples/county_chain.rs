use emerge::graph::{build_graph, render_graph};
use hecs::World;
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
    let mut world = World::new();
    let graph = build_graph();

    render_graph(&mut world, graph).await;
}
