use crate::graph_builder::build_graph;
use crate::graph_renderer::{render, Mass, NodeColor, Position, Size};
use hecs::World;
use macroquad::prelude::*;

mod graph_builder;
mod graph_renderer;
use random::prelude::*;

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
    let mut rng = random::rng();

    for _node in graph.nodes {
        let renderable_node = (
            Mass { mass: 1.0 },
            Position {
                x: rng.random_range(0.0..screen_width()),
                y: rng.random_range(0.0..screen_height()),
            },
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
