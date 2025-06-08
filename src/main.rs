use crate::graph_builder::build_graph;
use crate::graph_renderer::{render, Mass, NodeColor, Position, Size};
use hecs::World;
use macroquad::prelude::*;

mod graph_builder;
mod graph_renderer;

fn window_conf() -> Conf {
    Conf {
        window_title: "Circle Resizing Examples".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

fn physics_update(world: World) {}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new();
    let graph = build_graph();

    for _node in graph.nodes {
        // TODO: construct each renderable node from the graph
        let renderable_node = (
            Mass { mass: 1.0 },
            Position { x: 100.0, y: 100.0 },
            Size { radius: 15.0 },
            NodeColor { color: BLACK },
        );

        world.spawn(renderable_node);
    }

    loop {
        render(&mut world);

        next_frame().await
    }
}
